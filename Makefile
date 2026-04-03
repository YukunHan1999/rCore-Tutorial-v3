DOCKER_TAG ?= rcore-tutorial-v3:latest
.PHONY: docker build_docker
	
run:
	docker exec -it rcore-tutorial-v3 bash

docker:
	docker run -it -v ${PWD}:/mnt -w /mnt --name rcore-tutorial-v3 ${DOCKER_TAG}

build_docker: 
	docker build -t ${DOCKER_TAG} --target build .

fmt:
	cd easy-fs; cargo fmt; cd ../easy-fs-fuse cargo fmt; cd ../os ; cargo fmt; cd ../user; cargo fmt; cd ..

