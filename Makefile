## OpenAPI
OPENAPI_GENERATOR_VERSION=v5.3.1
OPENAPI_CONTAINER_NAME=resolve-openapi

openapi-gen: clean-openapi-docker-volume openapi-rust

openapi-rust:
	docker create -v /spec --name openapi-generator-volume openapitools/openapi-generator-cli:${OPENAPI_GENERATOR_VERSION} /bin/true
	docker cp ${PWD}/openapi.yaml openapi-generator-volume:/spec
	docker run --workdir /spec --volumes-from openapi-generator-volume --name ${OPENAPI_CONTAINER_NAME} --rm openapitools/openapi-generator-cli:${OPENAPI_GENERATOR_VERSION} \
	sh -c 'docker-entrypoint.sh generate -i /spec/openapi.yaml -g rust --additional-properties=packageName=todo -o /spec/openapi/todo/resolved'
	rm -rf ${PWD}/src/openapi/todo
	docker cp openapi-generator-volume:/spec/openapi/todo/resolved ${PWD}/src/openapi/todo
	cargo fmt --all --manifest-path ./Cargo.toml

clean-openapi-docker-volume:
	docker rm openapi-generator-volume --force 2>&1 > /dev/null || true

generate-entity:
	sea-orm-cli generate entity -u mysql://root:password@127.0.0.1/todo -o src/entity --with-serde both

migrate_dry:
	docker compose run mysqldef mysqldef -p password -h host.docker.internal --dry-run --file=/schema/schema.sql todo

migrate:
	docker compose run mysqldef mysqldef -p password -h host.docker.internal --file=/schema/schema.sql todo
