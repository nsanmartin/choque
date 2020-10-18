FROM rust

RUN apt-get -yqq update
RUN apt-get -yqq install curl

# wasm-pack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# cargo-generate
RUN apt-get -yqq install pkg-config libssl-dev git;	\
	cargo install cargo-generate;
ENV USER root

# nodejs
RUN apt-get -yqq install gnupg;	\
	curl -sL https://deb.nodesource.com/setup_10.x | bash -; \
	apt-get install -yqq nodejs


WORKDIR /app
COPY . .

RUN npm --prefix ./www install ./www
RUN wasm-pack build
EXPOSE 8080
CMD [ "npm", "--prefix", "www", "start" ]
#RUN cd www && npm start
