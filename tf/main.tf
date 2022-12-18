provider "aws" {
  region = "ca-central-1"
}

data "aws_vpc" "default" {
  default = true
}

resource "aws_db_instance" "eda-fun" {
  identifier             = "eda-fun"
  db_name = "edafun"
  instance_class         = "db.t4g.micro"
  allocated_storage      = 5
  max_allocated_storage = 5
  engine                 = "postgres"
  engine_version         = "14"
  skip_final_snapshot    = true
  publicly_accessible    = true
  vpc_security_group_ids = [aws_security_group.rds_sg.id]
  username               = "edafunadmin"
  password               = "password"
}

resource "aws_security_group" "rds_sg" {
  name        = "rds_sg"
  description = "Security group for RDS instances"
  vpc_id      = "${data.aws_vpc.default.id}"

  ingress {
    from_port   = 5432
    to_port     = 5432
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }
}