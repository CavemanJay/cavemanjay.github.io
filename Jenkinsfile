pipeline {
    agent {
        docker {
            image 'cimg/rust:1.67.1-node'
        }
    }
    stages {
        // stage('Setup') {
        //     steps {
        //         sh 'cargo install trunk'
        //     }
        // }
        stage('Build') {
            steps {
                // sh 'trunk build --release'
                sh 'cargo build --release'
                archiveArtifacts artifacts: '/target/**/*.*', fingerprint: true
            }
        }
    }
}
