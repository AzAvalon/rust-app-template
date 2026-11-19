variable "BAKE_CONTEXT" {
  default = "."
}

variable "DOCKERFILE" {
  default = "dockerfiles/Dockerfile"
}

variable "DOCKER_IMAGE" {
  default = "ghcr.io/azavalon/rust-app-template"
}

variable "DOCKER_TAG_PRIMARY" {
  default = "latest"
}

variable "DOCKER_TAG_EXTRA" {
  default = ""
}

variable "DOCKER_PLATFORMS" {
  default = "linux/amd64,linux/arm64"
}

target "_common" {
  context    = BAKE_CONTEXT
  dockerfile = DOCKERFILE
  platforms  = split(",", DOCKER_PLATFORMS)
  provenance = false
  secret = [
    "id=git-credentials,env=GIT_CREDENTIALS",
  ]
}

target "production" {
  inherits = ["_common"]
  target   = "production"
  tags = concat(
    ["${DOCKER_IMAGE}:${DOCKER_TAG_PRIMARY}"],
    DOCKER_TAG_EXTRA != "" ? ["${DOCKER_IMAGE}:${DOCKER_TAG_EXTRA}"] : []
  )
}

group "default" {
  targets = ["production"]
}
