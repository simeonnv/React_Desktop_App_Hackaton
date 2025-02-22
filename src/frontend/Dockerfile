FROM node:lts AS build
WORKDIR /app
COPY package*.json ./
RUN npm install
COPY . .
# RUN npm run build


# FROM nginx:alpine AS runtime
# COPY ./nginx.conf /etc/nginx/nginx.conf
# COPY --from=build /app/dist /usr/share/nginx/html

# CMD ["node", "./dist/server/entry.mjs"]
CMD ["npm", "run", "dev"]
EXPOSE 7004 