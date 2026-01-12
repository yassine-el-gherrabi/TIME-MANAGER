#!/bin/bash
# ==============================================
# Hetzner Server Setup Script for Time Manager
# Run this script on a fresh Ubuntu 24.04 server
# ==============================================

set -e

echo "🚀 Starting Time Manager server setup..."

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Update system
echo -e "${YELLOW}📦 Updating system packages...${NC}"
apt update && apt upgrade -y

# Install Docker
echo -e "${YELLOW}🐳 Installing Docker...${NC}"
apt install -y ca-certificates curl gnupg
install -m 0755 -d /etc/apt/keyrings
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | gpg --dearmor -o /etc/apt/keyrings/docker.gpg
chmod a+r /etc/apt/keyrings/docker.gpg

echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu \
  $(. /etc/os-release && echo "$VERSION_CODENAME") stable" | \
  tee /etc/apt/sources.list.d/docker.list > /dev/null

apt update
apt install -y docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin

# Start Docker
systemctl enable docker
systemctl start docker

# Configure firewall
echo -e "${YELLOW}🔒 Configuring firewall...${NC}"
apt install -y ufw
ufw default deny incoming
ufw default allow outgoing
ufw allow 22/tcp    # SSH
ufw allow 80/tcp    # HTTP
ufw allow 443/tcp   # HTTPS
ufw --force enable

# Create app directory
echo -e "${YELLOW}📁 Creating application directory...${NC}"
mkdir -p /opt/timemanager/infrastructure/traefik

# Set permissions
chown -R root:root /opt/timemanager

# Verify installation
echo -e "${GREEN}✅ Setup complete!${NC}"
echo ""
echo "Docker version:"
docker --version
echo ""
echo "Docker Compose version:"
docker compose version
echo ""
echo "Firewall status:"
ufw status
echo ""
echo -e "${GREEN}🎉 Server is ready for deployment!${NC}"
echo ""
echo "Next steps:"
echo "1. Configure GitHub secrets (HETZNER_HOST, HETZNER_SSH_KEY, POSTGRES_PASSWORD)"
echo "2. Configure GitHub variables (DOMAIN, ACME_EMAIL, VITE_API_BASE_URL)"
echo "3. Push to master branch to trigger deployment"
