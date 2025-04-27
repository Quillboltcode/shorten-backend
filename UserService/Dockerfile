# Use official .NET SDK for build
FROM mcr.microsoft.com/dotnet/sdk:8.0 AS build

# Set working directory
WORKDIR /app

# Copy just the UserService project file and restore dependencies
COPY UserService/UserService.csproj ./UserService/
WORKDIR /app/UserService
RUN dotnet restore

# Copy only the UserService files (not the entire solution)
COPY UserService/ ./

# OPTIONAL: install dotnet-ef CLI to generate migrations
# RUN dotnet tool install --global dotnet-ef && \
#     export PATH="$PATH:/root/.dotnet/tools" && \
#     dotnet ef migrations add InitialCreate --context UserDbContext --output-dir Data/Migrations || echo "Migration already exists"
# Build the application
RUN dotnet publish -c Release -o /publish

# Use a smaller runtime image for final deployment
FROM mcr.microsoft.com/dotnet/aspnet:8.0 AS runtime

# Set working directory inside the runtime container
WORKDIR /app
ENV ASPNETCORE_URLS=http://+:8082
# Copy built application from the build stage
COPY --from=build /publish .

# Expose the application's listening port
EXPOSE 8082

# Set the entry point
ENTRYPOINT ["dotnet", "UserService.dll"]