> The ARG instruction defines a variable that users can pass at build-time to the builder with the docker build command using the --build-arg <varname>=<value> flag. If a user specifies a build argument that was not defined in the Dockerfile, the build outputs a warning. 

https://docs.docker.com/engine/reference/builder/#automatic-platform-args-in-the-global-scope


**Build-in arguments** you can use in DockerFile without passing in build:

TARGETPLATFORM - platform of the build result. Eg linux/amd64, linux/arm/v7, windows/amd64.

TARGETOS - OS component of TARGETPLATFORM

TARGETARCH - architecture component of TARGETPLATFORM

TARGETVARIANT - variant component of TARGETPLATFORM

BUILDPLATFORM - platform of the node performing the build.

BUILDOS - OS component of BUILDPLATFORM

BUILDARCH - architecture component of BUILDPLATFORM

BUILDVARIANT - variant component of BUILDPLATFORM
