interface DockerContainerPort {
    IP: string;
    PrivatePort: number;
    PublicPort: number;
    Type: string;
  }
  
  interface DockerContainer {
    id: string;
    names: string[];
    image: string;
    imageId: string;
    command: string;
    created: number;
    ports: DockerContainerPort[];
    sizeRw: number;
    sizeRootFs: number;
    state: string;
    status: string;
  }
  
  interface DockerContainerResponse {
    status: string;
    data: DockerContainer[];
  }