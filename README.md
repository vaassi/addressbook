#### useful commands
delete data.sqlite
sea migrate up
sea generate entity -o entity/src/entities

#### create custom ad attribute
oidgen.vbs

#### compile project
cargo make

#### build docker image
docker build -t addressbook:0.1.0 .

#### run docker container
docker stop ab && docker rm ab
docker run -d --name ab --env-file=.env.example \
    -p 3000:3000 \
    -v ./data/db:/opt/app/db \
    -v ./data/assets:/opt/app/static/assets \
    addressbook:0.1.0
