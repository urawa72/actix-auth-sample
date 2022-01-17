redis:
	docker run -d -it --rm --name actix_redis_session -p 6379:6379 redis:latest

stop:
	docker stop actix_redis_session

cli:
	docker exec -it actix_redis_session redis-cli
