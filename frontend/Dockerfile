FROM node:18-alpine

# Set the working directory
WORKDIR /app

# Copy package.json and package-lock.json to the container
COPY package.json ./

# Copy the entire app directory to the container
COPY . .

# Expose the default React development port (usually 3000)
EXPOSE 5173

# Start the React development server
CMD ["yarn", "dev"]