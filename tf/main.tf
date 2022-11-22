provider "aws" {}
resource "aws_db_instance" "eda-fun" {
  allocated_storage    = 10
  engine               = "postgres"
  engine_version       = "14.1"
  instance_class       = "db.t4g.micro"
  name                 = "eda-fun"
  username             = "eda"
  password             = "admin"
  parameter_group_name = "test-instance-postgres-14.1"
}