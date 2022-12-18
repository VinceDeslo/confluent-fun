provider "aws" {
  region = "ca-central-1"
}

resource "aws_rds_cluster" "eda-fun" {
  cluster_identifier = "eda-fun-cluster"
  engine             = "aurora-postgresql"
  engine_version     = "14.0"
  availability_zones = ["ca-central-1a", "ca-central-1b"]
  backup_retention_period = 5
  port = 5432
  master_username = "admin"
  master_password = "password"
  skip_final_snapshot = true
  vpc_security_group_ids = [aws_security_group.rds_sg.id]
}

resource "aws_security_group" "rds_sg" {
  name        = "rds_sg"
  description = "Security group for RDS instances"
  vpc_id      = aws_vpc.vpc.id

  ingress {
    from_port   = 5432
    to_port     = 5432
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }
}

resource "aws_vpc" "vpc" {
  cidr_block = "10.0.0.0/16"

  tags = {
    Name = "eda-fun-vpc"
  }
}
