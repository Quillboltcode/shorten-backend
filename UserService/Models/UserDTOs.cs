using System;
using System.ComponentModel.DataAnnotations;
namespace UserService.Models;

public class UserDto
{
    public Guid Id { get; set; }  
    [Required]
    [EmailAddress]
    public string Email { get; set; } = string.Empty;
    public string? FullName { get; set; }
    public DateTime CreatedAt { get; set; }  
}

    public class UserCreateDto
{
    [Required]
    [EmailAddress]
    public string Email { get; set; } = string.Empty;
    [Required]
    public string PasswordHash { get; set; } = string.Empty;
    [MaxLength(100, ErrorMessage = "Full name can only be at most 100 characters")]
    public string? FullName { get; set; }
}

