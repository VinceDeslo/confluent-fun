provider "aws" {}

locals {
  name    = "edafun"
  region  = "ca-central-1"
}

resource "aws_db_instance" "edafun" {
  identifier           = local.name
  engine               = "postgres"
  engine_version       = "14.3"
  instance_class       = "db.t4g.micro"
  
  db_name              = "edafun"
  username             = "eda"
  password             = "edafunadmin"
  port                 = 5432

  allocated_storage    = 10
  skip_final_snapshot  = true
  
  db_subnet_group_name = module.vpc.database_subnet_group
  vpc_security_group_ids = [module.security_group.security_group_id]
}

module "vpc" {
  source  = "terraform-aws-modules/vpc/aws"
  version = "~> 3.0"

  name = local.name
  cidr = "10.99.0.0/18"

  azs              = ["${local.region}a", "${local.region}b"]
  public_subnets   = ["10.99.0.0/24", "10.99.1.0/24", "10.99.2.0/24"]
  private_subnets  = ["10.99.3.0/24", "10.99.4.0/24", "10.99.5.0/24"]
  database_subnets = ["10.99.7.0/24", "10.99.8.0/24", "10.99.9.0/24"]

  create_database_subnet_group       = true
  create_database_subnet_route_table = true
}

module "security_group" {
  source  = "terraform-aws-modules/security-group/aws"
  version = "~> 4.0"

  name        = local.name
  description = "EDA fun project security group"
  vpc_id      = module.vpc.vpc_id

  ingress_with_cidr_blocks = [
    {
      from_port = 5432
      to_port = 5432
      protocol = "tcp"
      description = "Public access"
      cidr_blocks = "0.0.0.0/0"
    },
    {
      from_port   = 5432
      to_port     = 5432
      protocol    = "tcp"
      description = "PostgreSQL access from within VPC"
      cidr_blocks = module.vpc.vpc_cidr_block
    }
  ]
}