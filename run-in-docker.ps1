$image = "rias_os/builder"
$pwd = $PSScriptRoot.Replace("\", "/")
$linux_pwd = "/host_mnt/" + $pwd.Replace(":", "").ToLower()

if ($args[0] -eq "init") {
    docker build -t $image -f Dockerfile.build .
    Write-Host "=> Image $image is now available"
    Exit
}

docker run -it --rm --volume /var/run/docker.sock:/var/run/docker.sock --volume ${pwd}:${linux_pwd} -w ${linux_pwd} -e CARGO_HOME=${linux_pwd}/.cache $image $args
