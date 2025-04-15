# Set your Docker Hub username and project (folder) name
$DockerUser = "quillbolt"
$ProjectName = "rustservice"  # This is usually your folder name or Docker Compose project name

# List of services to tag and push
$Services = @(
    "shortener-service",
    "redirect-service",
    "user-service",
    "api-gateway"
)

foreach ($Service in $Services) {
    $LocalImage = "$ProjectName" + "_" + "$Service"
    $RemoteImage = "$DockerUser/$Service:latest"

    Write-Host "➡️  Tagging $LocalImage as $RemoteImage"
    docker tag $LocalImage $RemoteImage

    Write-Host "🚀 Pushing $RemoteImage"
    docker push $RemoteImage
}

Write-Host "✅ All images pushed to Docker Hub."
