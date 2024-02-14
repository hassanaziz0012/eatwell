# /bin/bash
docker run -v .:/app -it -p 3000:3000 -p 8081:8081 eatwell

# -v .:/app will allow you to make changes in current dir and have them reflected instantly in container.