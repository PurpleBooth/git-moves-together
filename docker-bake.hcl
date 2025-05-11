target "bins" {
  name = "bins-${join("-", split("/", item.TARGETPLAFORM))}"
  dockerfile = "Dockerfile.bins"

  args = {
    TARGETPLAFORM = "${item.TARGETPLAFORM}"
  }
  output = ["type=local,dest=target/bins/${item.TARGETPLAFORM}"]

  secret = [
    "type=env,id=GPG_PRIVATE_KEY",
    "type=env,id=GPG_PASSPHRASE",
  ]

  matrix = {
    item = [
      {
        TARGETPLAFORM = "linux/amd64"
      },
      {
        TARGETPLAFORM = "linux/arm64"
      },
      {
        TARGETPLAFORM = "alpine/amd64"
      },
      {
        TARGETPLAFORM = "alpine/arm64"
      },
      {
        TARGETPLAFORM = "darwin/amd64"
      },
      {
        TARGETPLAFORM = "darwin/arm64"
      },
      {
        TARGETPLAFORM = "windows/amd64"
      },
      {
        TARGETPLAFORM = "windows/arm64"
      }
    ]
  }
}


target "docker" {

  attest = [
    "type=provenance,mode=max",
    "type=sbom"
  ]

  platform = ["alpine/amd64", "alpine/arm64"]

  dockerfile = "Dockerfile.container"
}

group "default" {
  targets = ["bins", "docker"]
}
