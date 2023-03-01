pipeline {
    agent {
        docker {
            image 'cimg/rust:1.67.1-node'
        }
    }
    stages {
        stage('Setup') {
            steps {
                sh 'cargo install trunk'
            }
        }
        stage('Build') {
            steps {
                sh 'trunk build --release'
                archiveArtifacts artifacts: '/dist/**/*.*', fingerprint: true
            }
        }
    }
}
