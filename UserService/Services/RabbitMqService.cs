using RabbitMQ.Client;
using System;
using System.Text;
using System.Threading;
using System.Threading.Tasks;

namespace UserService.Services;

public class RabbitMqService : IAsyncDisposable
{
    private readonly IConnection _connection;
    private readonly IChannel _channel;

    public RabbitMqService()
    {
        var factory = new ConnectionFactory()
        {
            HostName = "rabbitmq",
            UserName = "guest",
            Password = "password",
            ClientProvidedName = "app:user-service:event-producer"
        };

        _connection = factory.CreateConnectionAsync().GetAwaiter().GetResult();
        _channel = _connection.CreateChannelAsync().GetAwaiter().GetResult();
        
        _channel.ExchangeDeclareAsync(exchange: "user_events", type: ExchangeType.Fanout).GetAwaiter().GetResult();
    }

    public async Task PublishUserEventAsync(string eventType, string message, CancellationToken cancellationToken = default)
    {
        var body = Encoding.UTF8.GetBytes(message);

        var props = new BasicProperties
        {
            ContentType = "application/json",
            DeliveryMode = DeliveryModes.Persistent
        };

        await _channel.BasicPublishAsync(
            exchange: "user_events",
            routingKey: "",
            mandatory: false,
            basicProperties: props,
            body: body
        );
    }

    public async ValueTask DisposeAsync()
    {
        if (_channel != null)
        {
            await _channel.CloseAsync();
        }
        if (_connection != null)
        {
            await _connection.CloseAsync();
        }
    }
}
