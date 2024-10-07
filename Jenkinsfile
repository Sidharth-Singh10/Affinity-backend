pipeline {
    agent any

    environment {
        DOCKERHUB_CREDENTIALS = credentials('eb9c1cbf-8638-4a36-b866-dd6beb6471b0')
        BACKEND_IMAGE = 'sidharthsingh7/rusty_backend'
        DOCKER_TAG = 'latest'
        DEPLOY_URL = 'http://ec2-3-7-69-234.ap-south-1.compute.amazonaws.com:3002/webhook'
    }

    stages {
        stage('Build') {
            steps {
                script {
                    try {
                        echo 'Building the Docker image...'
                        sh "docker build -t ${BACKEND_IMAGE}:${DOCKER_TAG} ."
                    } catch (Exception e) {
                        error "Docker build failed: ${e.message}"
                    }
                }
            }
        }

        stage('Push') {
            steps {
                script {
                    try {
                        echo 'Pushing Docker image to DockerHub...'
                        docker.withRegistry('https://index.docker.io/v1/', DOCKERHUB_CREDENTIALS) {
                            def backendImage = docker.image("${BACKEND_IMAGE}:${DOCKER_TAG}")
                            backendImage.push()
                        }
                    } catch (Exception e) {
                        error "Docker push failed: ${e.message}"
                    }
                }
            }
        }

        stage('Deploy') {
            steps {
                script {
                    try {
                        echo 'Triggering deployment...'
                        sh "curl -X POST '${DEPLOY_URL}'"
                    } catch (Exception e) {
                        error "Deployment failed: ${e.message}"
                    }
                }
            }
        }
    }

    post {
        success {
            echo 'Pipeline executed successfully!'
        }
        failure {
            echo 'Pipeline execution failed. Please check the logs for details.'
        }
        always {
            echo 'Cleaning up...'
            sh 'docker system prune -f'
        }
    }
}
