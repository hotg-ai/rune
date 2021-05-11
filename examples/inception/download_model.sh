curl \
 -L https://storage.googleapis.com/download.tensorflow.org/models/inception_v1_224_quant_20181026.tgz \
  -o /tmp/inception_v1_224_quant.tgz
mkdir /tmp/inception
 tar -xf /tmp/inception_v1_224_quant.tgz --directory /tmp/inception
ls /tmp/inception

cp /tmp/inception/inception_v1_224_quant.tflite inception_v1_224_quant.tflite

