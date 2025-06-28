# 🚀 Deployment Guide

This guide covers various deployment options for the SNEK and EmojiVision applications.

## 📋 Prerequisites

- Docker & Docker Compose (recommended)
- Node.js 16+ (for local SNEK development)
- Modern web browser
- HTTPS setup (required for EmojiVision camera access in production)

## 🐳 Docker Deployment (Recommended)

### Quick Start
```bash
# Clone the repository
git clone <repository-url>
cd repos/

# Start both applications
docker-compose up -d

# Access applications
# SNEK: http://localhost:3000
# EmojiVision: http://localhost:3001
```

### Production Deployment
```bash
# Build production images
docker build -f Dockerfile.snek -t snek-game:latest .
docker build -f Dockerfile.emojivision -t emojivision:latest .

# Deploy with docker-compose
docker-compose -f docker-compose.yml up -d

# Or deploy individual services
docker run -d --name snek-production -p 3000:80 --restart unless-stopped snek-game:latest
docker run -d --name emojivision-production -p 3001:80 --restart unless-stopped emojivision:latest
```

### With Reverse Proxy (Traefik)
```bash
# Start with Traefik reverse proxy
docker-compose --profile traefik up -d

# Access via custom domains
# SNEK: http://snek.localhost
# EmojiVision: http://emojivision.localhost
# Traefik Dashboard: http://localhost:8080
```

## 🌐 Cloud Deployment

### AWS ECS/Fargate
```bash
# Build and push to ECR
aws ecr get-login-password --region us-east-1 | docker login --username AWS --password-stdin <account>.dkr.ecr.us-east-1.amazonaws.com
docker build -f Dockerfile.snek -t <account>.dkr.ecr.us-east-1.amazonaws.com/snek-game:latest .
docker push <account>.dkr.ecr.us-east-1.amazonaws.com/snek-game:latest

# Deploy with ECS service
aws ecs create-service --cluster <cluster-name> --service-name snek-game --task-definition snek-task --desired-count 2
```

### Google Cloud Run
```bash
# Build and deploy SNEK
gcloud builds submit --tag gcr.io/PROJECT-ID/snek-game --file Dockerfile.snek
gcloud run deploy snek-game --image gcr.io/PROJECT-ID/snek-game --platform managed --port 80

# Build and deploy EmojiVision
gcloud builds submit --tag gcr.io/PROJECT-ID/emojivision --file Dockerfile.emojivision
gcloud run deploy emojivision --image gcr.io/PROJECT-ID/emojivision --platform managed --port 80
```

### Azure Container Instances
```bash
# Create resource group
az group create --name games-rg --location eastus

# Deploy SNEK
az container create \
  --resource-group games-rg \
  --name snek-game \
  --image <registry>/snek-game:latest \
  --dns-name-label snek-game-unique \
  --ports 80

# Deploy EmojiVision
az container create \
  --resource-group games-rg \
  --name emojivision \
  --image <registry>/emojivision:latest \
  --dns-name-label emojivision-unique \
  --ports 80
```

### Kubernetes
```yaml
# k8s-deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: snek-game
spec:
  replicas: 3
  selector:
    matchLabels:
      app: snek-game
  template:
    metadata:
      labels:
        app: snek-game
    spec:
      containers:
      - name: snek-game
        image: snek-game:latest
        ports:
        - containerPort: 80
---
apiVersion: v1
kind: Service
metadata:
  name: snek-service
spec:
  selector:
    app: snek-game
  ports:
  - port: 80
    targetPort: 80
  type: LoadBalancer
```

```bash
# Deploy to Kubernetes
kubectl apply -f k8s-deployment.yaml
```

## 🖥️ Traditional Server Deployment

### SNEK (Node.js Build)
```bash
# On server
cd snek/
npm install --production
npm run build

# Serve with Nginx
sudo cp -r dist/* /var/www/html/snek/

# Nginx configuration
server {
    listen 80;
    server_name snek.yourdomain.com;
    root /var/www/html/snek;
    index index.html;
    
    location / {
        try_files $uri $uri/ /index.html;
    }
}
```

### EmojiVision (Static Files)
```bash
# Copy files to web server
sudo cp -r emojivision/* /var/www/html/emojivision/

# Nginx configuration for camera access
server {
    listen 443 ssl;
    server_name emojivision.yourdomain.com;
    root /var/www/html/emojivision;
    
    ssl_certificate /path/to/cert.pem;
    ssl_certificate_key /path/to/key.pem;
    
    location / {
        try_files $uri $uri/ /index.html;
    }
}
```

## 🔒 Security Considerations

### HTTPS Setup (Required for EmojiVision)
```bash
# Let's Encrypt with Certbot
sudo certbot --nginx -d emojivision.yourdomain.com

# Or with Docker
docker run -it --rm --name certbot \
  -v "/etc/letsencrypt:/etc/letsencrypt" \
  -v "/var/lib/letsencrypt:/var/lib/letsencrypt" \
  certbot/certbot certonly --standalone -d emojivision.yourdomain.com
```

### Content Security Policy
```nginx
# Add to Nginx configuration
add_header Content-Security-Policy "default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline' fonts.googleapis.com; font-src 'self' fonts.gstatic.com; media-src 'self' blob:; connect-src 'self';" always;
```

### Security Headers
```nginx
add_header X-Frame-Options "SAMEORIGIN" always;
add_header X-Content-Type-Options "nosniff" always;
add_header X-XSS-Protection "1; mode=block" always;
add_header Referrer-Policy "strict-origin-when-cross-origin" always;
```

## 📊 Monitoring & Health Checks

### Docker Health Checks
Both Dockerfiles include health checks:
```dockerfile
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD curl -f http://localhost/ || exit 1
```

### Nginx Health Endpoint
```nginx
location /health {
    access_log off;
    return 200 "healthy\n";
    add_header Content-Type text/plain;
}
```

### Application Monitoring
```bash
# Monitor container logs
docker-compose logs -f snek
docker-compose logs -f emojivision

# Monitor resource usage
docker stats

# Monitor application health
curl http://localhost:3000/health
curl http://localhost:3001/health
```

## 🔧 Environment Configuration

### Environment Variables
```bash
# .env file for docker-compose
NODE_ENV=production
NGINX_PORT=80
SNEK_PORT=3000
EMOJIVISION_PORT=3001
```

### Production Optimizations
```nginx
# Gzip compression
gzip on;
gzip_types text/plain text/css application/json application/javascript text/xml application/xml application/xml+rss text/javascript;

# Cache static assets
location ~* \.(js|css|png|jpg|jpeg|gif|ico|svg|woff|woff2|ttf|eot)$ {
    expires 1y;
    add_header Cache-Control "public, immutable";
}
```

## 🚨 Troubleshooting

### Common Issues

#### Camera Access Not Working (EmojiVision)
- Ensure HTTPS is configured
- Check browser permissions
- Verify Content-Security-Policy allows media-src

#### Audio Not Working (SNEK)
- Check Web Audio API support
- Ensure user interaction before audio starts
- Verify no Content-Security-Policy blocking

#### Performance Issues
- Monitor container resource limits
- Check Nginx access logs for errors
- Verify static asset caching

### Debug Commands
```bash
# Check container status
docker-compose ps

# View logs
docker-compose logs --tail=50 snek

# Test endpoints
curl -I http://localhost:3000/
curl -I http://localhost:3001/

# Check container resource usage
docker stats --no-stream
```

## 📈 Scaling

### Horizontal Scaling
```yaml
# docker-compose.yml
services:
  snek:
    scale: 3
  nginx-lb:
    image: nginx:alpine
    volumes:
      - ./nginx-lb.conf:/etc/nginx/nginx.conf
```

### Load Balancing
```nginx
# nginx-lb.conf
upstream snek_backend {
    server snek_1:80;
    server snek_2:80;
    server snek_3:80;
}

server {
    listen 80;
    location / {
        proxy_pass http://snek_backend;
    }
}
```

## 📝 Deployment Checklist

### Pre-deployment
- [ ] Test applications locally
- [ ] Run test suites (SNEK)
- [ ] Verify Docker builds
- [ ] Check security configurations
- [ ] Prepare environment variables

### Deployment
- [ ] Deploy to staging environment
- [ ] Run smoke tests
- [ ] Monitor application logs
- [ ] Verify health checks
- [ ] Test in production browser

### Post-deployment
- [ ] Monitor application performance
- [ ] Check error logs
- [ ] Verify analytics/monitoring
- [ ] Document any issues
- [ ] Update team on deployment status

---

For questions or issues, please refer to the project README.md files or create an issue in the repository.