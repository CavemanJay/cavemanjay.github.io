pipeline {
    agent any
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
