using Microsoft.AspNetCore.Mvc;
using Microsoft.EntityFrameworkCore;
using UserService.Data;
using UserService.Models;
using System.IdentityModel.Tokens.Jwt;
using System.Security.Claims;
using System.Text;
using Microsoft.IdentityModel.Tokens;
using Microsoft.AspNetCore.Authorization;

namespace UserService.Controllers;

// Endpoints:
///    GET  /users (Get All Users) 
//     POST /users (Register User)
//     POST /users/login (Login and Generate Token)
//     POST /users/logout (Logout and Invalidate Token)
//     POST /users/validate-token (Validate Token)
//     GET /users/{userId} (Get User Profile)
//     PUT /users/{userId} (Update User Profile)
//     PUT /users/{userId}/password (Change User Password)
//     DELETE /users/{userId} (Delete User Account)

[Route("api/users")]
[ApiController]
public class UserController : ControllerBase
{
    private readonly AppDbContext _context;
    private readonly ILogger<UserController> _logger;
    private readonly IConfiguration _configuration;

    public UserController(AppDbContext context, ILogger<UserController> logger, IConfiguration configuration)
    {
        _context = context;
        _logger = logger;
        _configuration = configuration;
    }

    // ✅ GET all users
    [HttpGet]
    // [Authorize]
    public async Task<ActionResult<IEnumerable<User>>> GetUsers()
    {
        _logger.LogInformation("Fetching all users.");
        var users = await _context.Users.ToListAsync();
        // Remove password hashes from response
        foreach (var user in users)
        {
            user.PasswordHash = string.Empty;
        }
        return users;
    }

    // ✅ GET user by ID
    [HttpGet("{id}")]
    [Authorize]
    public async Task<ActionResult<User>> GetUser(int id)
    {
        _logger.LogInformation($"Fetching user with ID: {id}");
        var user = await _context.Users.FindAsync(id);
        if (user == null) return NotFound();
        
        // Remove password hash from response
        user.PasswordHash = string.Empty;   
        return user;
    }

    // ✅ CREATE a new user (Register)
    [HttpPost]
    public async Task<ActionResult<User>> CreateUser(RegisterRequest request)
    {
        _logger.LogInformation("Creating a new user.");
        
        // Check if username already exists
        if (await _context.Users.AnyAsync(u => u.Username == request.Username))
        {
            return BadRequest("Username already exists");
        }
        
        var user = new User
        {
            Username = request.Username,
            Email = request.Email,
            PasswordHash = BCrypt.Net.BCrypt.HashPassword(request.Password)
        };
        
        _context.Users.Add(user);
        await _context.SaveChangesAsync();
        
        // Remove password hash from response
        user.PasswordHash = string.Empty;   
        return CreatedAtAction(nameof(GetUser), new { id = user.Id }, user);
    }

    // ✅ UPDATE a user
    [HttpPut("{id}")]
    [Authorize]
    public async Task<IActionResult> UpdateUser(int id, UpdateUserRequest request)
    {
        _logger.LogInformation($"Updating user with ID: {id}");
        
        var user = await _context.Users.FindAsync(id);
        if (user == null) return NotFound();
        
        // Check if the authenticated user is updating their own profile
        var userId = User.FindFirst(ClaimTypes.NameIdentifier)?.Value;
        if (userId != id.ToString())
        {
            return Forbid();
        }
        
        // Update user properties
        if (!string.IsNullOrEmpty(request.Email))
        {
            user.Email = request.Email;
        }
        
        // Add other properties to update as needed
        
        await _context.SaveChangesAsync();
        return NoContent();
    }

    // ✅ DELETE a user
    [HttpDelete("{id}")]
    [Authorize]
    public async Task<IActionResult> DeleteUser(int id)
    {
        _logger.LogInformation($"Deleting user with ID: {id}");
        
        var user = await _context.Users.FindAsync(id);
        if (user == null) return NotFound();
        
        // Check if the authenticated user is deleting their own account
        var userId = User.FindFirst(ClaimTypes.NameIdentifier)?.Value;
        if (userId != id.ToString())
        {
            return Forbid();
        }
        
        _context.Users.Remove(user);
        await _context.SaveChangesAsync();
        return NoContent();
    }

    // ✅ PUT a user password
    [HttpPut("{id}/password")]
    [Authorize]
    public async Task<IActionResult> UpdateUserPassword(int id, ChangePasswordRequest request)
    {
        _logger.LogInformation($"Updating password for user with ID: {id}");
        
        var user = await _context.Users.FindAsync(id);
        if (user == null) return NotFound();
        
        // Check if the authenticated user is updating their own password
        var userId = User.FindFirst(ClaimTypes.NameIdentifier)?.Value;
        if (userId != id.ToString())
        {
            return Forbid();
        }
        
        // Verify current password
        if (!BCrypt.Net.BCrypt.Verify(request.CurrentPassword, user.PasswordHash))
        {
            return BadRequest("Current password is incorrect");
        }
        
        // Hash the new password
        user.PasswordHash = BCrypt.Net.BCrypt.HashPassword(request.NewPassword);
        
        await _context.SaveChangesAsync();
        return NoContent();
    }

    [HttpPost("login")]
    public async Task<ActionResult> Login([FromBody] LoginRequest request)
    {
        _logger.LogInformation($"Attempting login for user: {request.Username}");
        var user = await _context.Users.FirstOrDefaultAsync(u => u.Username == request.Username);

        if (user == null || !BCrypt.Net.BCrypt.Verify(request.Password, user.PasswordHash))
        {
            _logger.LogWarning("Invalid login attempt.");
            return Unauthorized();
        }

        // Generate JWT token
        var token = GenerateJwtToken(user);
        return Ok(new { Token = token });
    }
    
    [HttpPost("logout")]
    [Authorize]
    public ActionResult Logout()
    {
        // Since JWT tokens are stateless, the client should discard the token
        // For additional security, you could implement a token blacklist
        _logger.LogInformation("User logged out");
        return Ok(new { Message = "Logged out successfully" });
    }
    
    [HttpPost("validate-token")]
    [Authorize]
    public ActionResult ValidateToken()
    {
        // If we reach here, the token is valid (due to [Authorize] attribute)
        var userId = User.FindFirst(ClaimTypes.NameIdentifier)?.Value;
        var username = User.FindFirst(ClaimTypes.Name)?.Value;
        
        return Ok(new { UserId = userId, Username = username, Valid = true });
    }

    private string GenerateJwtToken(User user)
    {
        // Retrieve JWT settings from the configuration
        var jwtSettings = _configuration.GetSection("JwtSettings");
        var secretKey = jwtSettings["SecretKey"] ?? throw new InvalidOperationException("JWT Secret Key is not configured");
        var issuer = jwtSettings["Issuer"] ?? "userservice";
        var audience = jwtSettings["Audience"] ?? "userservice_clients";
        var expiryMinutes = int.Parse(jwtSettings["ExpiryMinutes"] ?? "60");

        // Create a security key using the secret key from the settings
        var securityKey = new SymmetricSecurityKey(Encoding.UTF8.GetBytes(secretKey));
        
        // Create signing credentials using the security key and a hashing algorithm
        var credentials = new SigningCredentials(securityKey, SecurityAlgorithms.HmacSha256);

        // Define claims to be included in the JWT token
        var claims = new[]
        {
            new Claim(ClaimTypes.NameIdentifier, user.Id.ToString()), // User ID claim
            new Claim(ClaimTypes.Name, user.Username) // Username claim
        };

        // Create a new JWT token using the specified parameters
        var token = new JwtSecurityToken(
            issuer: issuer, // Token issuer
            audience: audience, // Token audience
            claims: claims, // Claims for the token
            expires: DateTime.Now.AddMinutes(expiryMinutes), // Expiration time
            signingCredentials: credentials // Signing credentials
        );

        // Write the token to a string and return it
        return new JwtSecurityTokenHandler().WriteToken(token);
    }
}

// Model classes


