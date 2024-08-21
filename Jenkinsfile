pipeline {
    agent any

    environment {
        DOCKERHUB_CREDENTIALS = credentials('eb9c1cbf-8638-4a36-b866-dd6beb6471b0')
        BACKEND_IMAGE = 'sidharthsingh7/rusty_backend'
        DOCKER_TAG = 'latest'
    }

    stages {
        stage('Build') {
            steps {
                script {
                    // Build the backend Docker image
                    sh 'docker build -t $BACKEND_IMAGE:$DOCKER_TAG .'
                }
            }
        }

        stage('Push') {
            steps {
                script {
                    docker.withRegistry('https://index.docker.io/v1/', DOCKERHUB_CREDENTIALS) {
                        def backendImage = docker.image("$BACKEND_IMAGE:$DOCKER_TAG")
                        backendImage.push()
                    }
                }
            }
        }
    }
}
