provider "aws" {}
resource "aws_db_instance" "edafun" {
  allocated_storage    = 10
  engine               = "postgres"
  engine_version       = "14.1"
  instance_class       = "db.t4g.micro"
  db_name              = "edafun"
  username             = "eda"
  password             = "edafunadmin"
  parameter_group_name = "test-instance-postgres-14.1"
}