cd scripts \
&& echo "Downloading jextract tool..." \
&& wget https://download.java.net/java/early_access/jextract/25/1/openjdk-25-jextract+1-1_linux-x64_bin.tar.gz \
&& echo "Done! Extracting..." \
&& tar -xvf openjdk-25-jextract+1-1_linux-x64_bin.tar.gz \
&& echo "Done! Cleaning up..." \
&& rm -rf openjdk-25-jextract+1-1_linux-x64_bin.tar.gz \
&& echo "jextract is ready to use!"