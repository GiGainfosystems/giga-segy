import groovy.json.JsonSlurperClassic;
import groovy.json.JsonOutput;

library 'giga'

def run_build() {
  try{
    giga.notifyStatus("INPROGRESS", "all", null);
    giga.mergeTarget(".")
    giga.mysh 'cp /var/lib/jenkins/.ssh/id_rsa .'
    def RUSTC_VERSION = giga.read_rustc_version();
    // `id jenkins -u` is reporting 0 here, so hard code it is.
    def jenkins_uid = 109
    // `id jenkins -g` is reporting 0 here, so hard code it is.
    def jenkins_gid = 116
    stage('Build Docker'){
      def image = docker.build(
        "gst3.0:debian-rust-segy-nightly-${RUSTC_VERSION}",
        "--build-arg=RUST_VERSION='${RUSTC_VERSION}' \
          --build-arg=USER_ID='${jenkins_uid}' \
          --build-arg=GROUP_ID='${jenkins_gid}' \
          -f build/docker/Dockerfile .")
      image.inside('-v /var/lib/jenkins/.cargo_jenkins/registry:/home/rust/.cargo/registry \
                    -v /var/lib/jenkins/.cargo_jenkins/git:/home/rust/.cargo/git \
                    -e LD_LIBRARY_PATH=/usr/local/lib/') {
        giga.mysh 'rustc --version'
        def no_url = '';
        stage('Rustfmt'){
          try {
            giga.mysh "cargo fmt --all -- --check"
          } catch(err) {
            def message = [text: "Formatting issues detected. Please run `cargo fmt --all` before you open a PR. Check console for specific issues."];
            giga.postComment(JsonOutput.toJson(message));
            giga.notifyStatus("FAILED", "all", "Formatting issues detected.")
          }
        }
        stage ('Checking License'){
          try {
            giga.mysh "cargo-deny check --hide-inclusion-graph licenses 2> licensecheck.txt"
          } catch (err) {
            // A new allowed license can be input inside deny.toml
            def readlicensecheck = readFile (file: 'licensecheck.txt')
            def licensemessage = [text: "${readlicensecheck}"]
            giga.postComment(JsonOutput.toJson(licensemessage));
            giga.notifyStatus("FAILED", "all", "License issues detected.")
          }
        }
        stage('Run tests'){
          giga.test_dir("giga-segy-core", "", 4, no_url, 'test', true);
          giga.test_dir("giga-segy-in", "", 4, no_url, 'test', true);
          giga.test_dir("giga-segy-in", "to_json", 4, no_url, 'test', true);
          giga.test_dir("giga-segy-out", "", 4, no_url, 'test', true);     
        }
      }
    }
  }catch (e){
    echo "${e}"
    // We need to post the general error message from Jenkins as an insurance on the special case error (DO-201) in the future
    giga.generalPostComment("${e}")
    giga.notifyStatus("FAILED", "all", null);
    throw e;
  }finally{
    def clippyWarningcheck = ""
    def clippyWarningreport = ""
    if ("${CWURL}" != "http://192.168.2.50:7990/rest/insights/latest/projects//repos//commits//reports/my.clippy.report") {
      if (fileExists ('clippyWarning')) {
        clippyWarningcheck = readFile (file: 'clippyWarning')
        clippyWarningreport = new JsonSlurperClassic().parseText(clippyWarningcheck)
        giga.mysh "sed -i s#'CLIPPYWARNINGAMOUNT'#${clippyWarningreport.size}#g qualitygate/clippyreport.json"
        giga.mysh "sed -i s#'CLIPPYRESULT'#'FAIL'#g qualitygate/clippyreport.json"
        giga.mysh "curl -s -u jenkins:jenkins -H 'Content-Type: application/json' -X PUT ${CWURL} -d @qualitygate/clippyreport.json "
      } else {
        giga.mysh "sed -i s#'CLIPPYWARNINGAMOUNT'#'0'#g qualitygate/clippyreport.json"
        giga.mysh "sed -i s#'CLIPPYRESULT'#'PASS'#g qualitygate/clippyreport.json"
        giga.mysh "curl -s -u jenkins:jenkins -H 'Content-Type: application/json' -X PUT ${CWURL} -d @qualitygate/clippyreport.json "
      }
    }
    giga.mysh "rm -rf clippyWarning content id_rsa"
  }
  giga.notifyStatus("SUCCESSFUL", "all", null);
}

return this;
