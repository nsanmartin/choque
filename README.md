Para compilar esto:
wasp-pack build

Para correr:
cd www
npm start

Docker
docker build --tag choque:1.0 .
docker run --publish 8000:8080 --detach --name ch choque:1.0
docker rm --force ch
