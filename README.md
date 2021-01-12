Para compilar esto:

cargo build
wasp-pack build

Para correr:

cd www
npm start


Docker (no lo pude hacer andar, supongo que hay q arreglar el dockerfile):

docker build --tag choque:1.0 .
docker run --publish 8000:8080 --detach --name ch choque:1.0
docker rm --force ch
