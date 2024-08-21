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
                    docker.withRegistry('https://index.docker.io/v1/', 'eb9c1cbf-8638-4a36-b866-dd6beb6471b0') {
                        def backendImage = docker.image("$BACKEND_IMAGE:$DOCKER_TAG")
                        backendImage.push()
                    }
                }
            }
        }
        stage('Deploy')
        {
            steps {
                steps {
                    def response = sh(
                        script: 'curl -s -o /dev/null -w "%{http_code}" -X POST "http://ec2-3-7-69-234.ap-south-1.compute.amazonaws.com:3002/webhook"',
                        returnStdout: true
                    ).trim()

                    if (response == '200') {
                        echo 'Deployment triggered successfully!'
                    } else {
                        error "Deployment failed with HTTP status code: ${response}"
                    }
                }
            }
        }
    }
}
