using UserService.Models;
using UserService.Data;
using System.Threading.Tasks;
using System.Text.Json;
using Microsoft.Extensions.Logging;

namespace UserService.Services;

public class UserService
{
    private readonly RabbitMqService _rabbitMqService;
    private readonly AppDbContext _dbContext;
    private readonly ILogger<UserService> _logger;

    public UserService(RabbitMqService rabbitMqService, AppDbContext dbContext, ILogger<UserService> logger)
    {
        _rabbitMqService = rabbitMqService;
        _dbContext = dbContext;
        _logger = logger;
    }

    // ✅ Create User Event
    public async Task CreateUser(UserCreateDto userDto)
    {
        var user = new User
        {
            Email = userDto.Email,
            PasswordHash = userDto.PasswordHash
        };

        _dbContext.Users.Add(user);
        await _dbContext.SaveChangesAsync();

        var userCreatedEvent = new
        {
            EventType = "UserCreated",
            UserId = user.Id,
            Email = user.Email
        };

        string message = JsonSerializer.Serialize(userCreatedEvent);
        await _rabbitMqService.PublishUserEventAsync("UserCreated", message);
        _logger.LogInformation("User created and event published: {Email}", user.Email);
    }

    // ✅ Update User Event
    public async Task UpdateUser(int id, User updatedUser)
    {
        var user = await _dbContext.Users.FindAsync(id);
        if (user == null) throw new Exception("User not found");

        user.Email = updatedUser.Email;
        user.Username = updatedUser.Username;

        await _dbContext.SaveChangesAsync();

        var userUpdatedEvent = new
        {
            EventType = "UserUpdated",
            UserId = user.Id,
            Email = user.Email
        };

        string message = JsonSerializer.Serialize(userUpdatedEvent);
        await _rabbitMqService.PublishUserEventAsync("UserUpdated", message);
        _logger.LogInformation("User updated and event published: {Email}", user.Email);
    }

    // ✅ Delete User Event
    public async Task DeleteUser(int id)
    {
        var user = await _dbContext.Users.FindAsync(id);
        if (user == null) throw new Exception("User not found");

        _dbContext.Users.Remove(user);
        await _dbContext.SaveChangesAsync();

        var userDeletedEvent = new
        {
            EventType = "UserDeleted",
            UserId = user.Id
        };

        string message = JsonSerializer.Serialize(userDeletedEvent);
        await _rabbitMqService.PublishUserEventAsync("UserDeleted", message);
        _logger.LogInformation("User deleted and event published: {UserId}", user.Id);
    }

    // ✅ Change User Password Event
    public async Task ChangeUserPassword(int id, string newPassword)
    {
        var user = await _dbContext.Users.FindAsync(id);
        if (user == null) throw new Exception("User not found");

        user.PasswordHash = newPassword;
        await _dbContext.SaveChangesAsync();

        var passwordChangedEvent = new
        {
            EventType = "UserPasswordChanged",
            UserId = user.Id
        };

        string message = JsonSerializer.Serialize(passwordChangedEvent);
        await _rabbitMqService.PublishUserEventAsync("UserPasswordChanged", message);
        _logger.LogInformation("User password changed and event published: {UserId}", user.Id);
    }
}
