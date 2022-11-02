docker stop $(docker ps -a --format {{.ID}})
docker volume rm $(docker volume ls -qf dangling=true)
docker system prune 

