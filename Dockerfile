FROM httpd:latest

# Setup Rust and transfer ownership to user daemon
RUN apt-get update;\
    apt-get -y install curl ca-certificates mingw-w64 gcc;\
    curl https://sh.rustup.rs -sSf | sh -s -- -y;\
    chown -R daemon:daemon /root;\
    chown -R daemon:daemon /usr

# Do the source thing
ENV PATH /root/.cargo/bin:$PATH

# Install stable toolchain for user daemon
USER daemon
RUN rustup default stable;\
    rustup target add x86_64-pc-windows-gnu
USER root

# Copy necessary windows dependency
RUN cd /usr/sbin/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-pc-windows-gnu/lib/;\
    cp crt2.o crt2.o.bak;\
    cp /usr/x86_64-w64-mingw32/lib/crt2.o ./

# Copy build files
RUN mkdir -p /usr/app
WORKDIR /usr/app
ADD Cargo.toml ./
ADD .cargo ./.cargo
ADD src ./src
RUN chmod +x -R /usr/app;\
    chown -R daemon:daemon /usr/app

# Copy apache stuff over
WORKDIR /
#Copy conf
COPY httpd.conf /usr/local/apache2/conf/httpd.conf
# Copy CGI
COPY file-scanner.cgi /usr/local/apache2/cgi-bin/

# Test build
USER daemon
WORKDIR /usr/app
ENV FILE_SCANNER_HASHES stuff
ENV FILE_SCANNER_DELIMITER ,
RUN cargo build --target x86_64-pc-windows-gnu --release
USER root
