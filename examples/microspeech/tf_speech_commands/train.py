# Copyright 2017 The TensorFlow Authors. All Rights Reserved.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
# ==============================================================================
r"""Simple speech recognition to spot a limited number of keywords.

This is a self-contained example script that will train a very basic audio
recognition model in TensorFlow. It downloads the necessary training data and
runs with reasonable defaults to train within a few hours even only using a CPU.
For more information, please see
https://www.tensorflow.org/tutorials/audio_recognition.

It is intended as an introduction to using neural networks for audio
recognition, and is not a full speech recognition system. For more advanced
speech systems, I recommend looking into Kaldi. This network uses a keyword
detection style to spot discrete words from a small vocabulary, consisting of
"yes", "no", "up", "down", "left", "right", "on", "off", "stop", and "go".

To run the training process, use:

bazel run tensorflow/examples/speech_commands:train

This will write out checkpoints to /tmp/speech_commands_train/, and will
download over 1GB of open source training data, so you'll need enough free space
and a good internet connection. The default data is a collection of thousands of
one-second .wav files, each containing one spoken word. This data set is
collected from https://aiyprojects.withgoogle.com/open_speech_recording, please
consider contributing to help improve this and other models!

As training progresses, it will print out its accuracy metrics, which should
rise above 90% by the end. Once it's complete, you can run the freeze script to
get a binary GraphDef that you can easily deploy on mobile applications.

If you want to train on your own data, you'll need to create .wavs with your
recordings, all at a consistent length, and then arrange them into subfolders
organized by label. For example, here's a possible file structure:

my_wavs >
  up >
    audio_0.wav
    audio_1.wav
  down >
    audio_2.wav
    audio_3.wav
  other>
    audio_4.wav
    audio_5.wav

You'll also need to tell the script what labels to look for, using the
`--wanted_words` argument. In this case, 'up,down' might be what you want, and
the audio in the 'other' folder would be used to train an 'unknown' category.

To pull this all together, you'd run:

bazel run tensorflow/examples/speech_commands:train -- \
--data_dir=my_wavs --wanted_words=up,down

"""
from __future__ import absolute_import
from __future__ import division
from __future__ import print_function

from tensorflow.python.ops import gen_audio_ops as audio_ops
from tensorflow.python.ops import io_ops
from tensorflow.python.platform import gfile
from tensorflow.python.util import compat

import numpy as np
import tensorflow as tf
import os, sys

from tensorflow.python.ops import io_ops
import input_data, models

try:
    from tensorflow.lite.experimental.microfrontend.python.ops import audio_microfrontend_op as frontend_op
except ImportError:
    frontend_op = None
    
from six.moves import xrange  # pylint: disable=redefined-builtin

    
tf.__version__

FLAGS = None


def train(audio_processor, model_settings, params):

    tf.compat.v1.logging.set_verbosity(params["verbosity"])

    # Start a new TensorFlow session.
    sess = tf.compat.v1.InteractiveSession()

    fingerprint_size = model_settings['fingerprint_size']
    label_count = model_settings['label_count']
    time_shift_samples = int(
        (params["time_shift_ms"] * params["sample_rate"]) / 1000)
    # Figure out the learning rates for each training phase. Since it's often
    # effective to have high learning rates at the start of training, followed by
    # lower levels towards the end, the number of steps and learning rates can be
    # specified as comma-separated lists to define the rate at each stage. For
    # example --how_many_training_steps=10000,3000 --learning_rate=0.001,0.0001
    # will run 13,000 training loops in total, with a rate of 0.001 for the first
    # 10,000, and 0.0001 for the final 3,000.
    training_steps_list = list(
        map(int, params["how_many_training_steps"].split(',')))
    learning_rates_list = list(map(float, params["learning_rate"].split(',')))
    if len(training_steps_list) != len(learning_rates_list):
        raise Exception(
            '--how_many_training_steps and --learning_rate must be equal length '
            'lists, but are %d and %d long instead' % (len(training_steps_list),
                                                       len(learning_rates_list)))

    input_placeholder = tf.compat.v1.placeholder(
        tf.float32, [None, fingerprint_size], name='fingerprint_input')
    if params["quantize"]:
        fingerprint_min, fingerprint_max = input_data.get_features_range(
            model_settings)
        fingerprint_input = tf.quantization.fake_quant_with_min_max_args(
            input_placeholder, fingerprint_min, fingerprint_max)
    else:
        fingerprint_input = input_placeholder

    logits, dropout_rate = models.create_model(
        fingerprint_input,
        model_settings,
        params["model_architecture"],
        is_training=True)

    # Define loss and optimizer
    ground_truth_input = tf.compat.v1.placeholder(
        tf.int64, [None], name='groundtruth_input')

    # Optionally we can add runtime checks to spot when NaNs or other symptoms of
    # numerical errors start occurring during training.
    control_dependencies = []
    if params["check_nans"]:
        checks = tf.compat.v1.add_check_numerics_ops()
        control_dependencies = [checks]

    # Create the back propagation and training evaluation machinery in the graph.
    with tf.compat.v1.name_scope('cross_entropy'):
        cross_entropy_mean = tf.compat.v1.losses.sparse_softmax_cross_entropy(
            labels=ground_truth_input, logits=logits)

    if params["quantize"]:
        try:
            tf.contrib.quantize.create_training_graph(quant_delay=0)
        except AttributeError as e:
            msg = e.args[0]
            msg += ('\n\n The --quantize option still requires contrib, which is not '
                    'part of TensorFlow 2.0. Please install a previous version:'
                    '\n    `pip install tensorflow<=1.15`')
            e.args = (msg,)
            raise e

    with tf.compat.v1.name_scope('train'), tf.control_dependencies(
            control_dependencies):
        learning_rate_input = tf.compat.v1.placeholder(
            tf.float32, [], name='learning_rate_input')
        if params["optimizer"] == 'gradient_descent':
            train_step = tf.compat.v1.train.GradientDescentOptimizer(
                learning_rate_input).minimize(cross_entropy_mean)
        elif params["optimizer"] == 'momentum':
            train_step = tf.compat.v1.train.MomentumOptimizer(
                learning_rate_input, .9,
                use_nesterov=True).minimize(cross_entropy_mean)
        else:
            raise Exception('Invalid Optimizer')
    predicted_indices = tf.argmax(input=logits, axis=1)
    correct_prediction = tf.equal(predicted_indices, ground_truth_input)
    confusion_matrix = tf.math.confusion_matrix(labels=ground_truth_input,
                                                predictions=predicted_indices,
                                                num_classes=label_count)
    evaluation_step = tf.reduce_mean(input_tensor=tf.cast(correct_prediction,
                                                          tf.float32))
    with tf.compat.v1.get_default_graph().name_scope('eval'):
        tf.compat.v1.summary.scalar('cross_entropy', cross_entropy_mean)
        tf.compat.v1.summary.scalar('accuracy', evaluation_step)

    global_step = tf.compat.v1.train.get_or_create_global_step()
    increment_global_step = tf.compat.v1.assign(global_step, global_step + 1)

    saver = tf.compat.v1.train.Saver(tf.compat.v1.global_variables())

    # Merge all the summaries and write them out to /tmp/retrain_logs (by default)
    merged_summaries = tf.compat.v1.summary.merge_all(scope='eval')
    train_writer = tf.compat.v1.summary.FileWriter(params["summaries_dir"] + '/train',
                                                   sess.graph)
    validation_writer = tf.compat.v1.summary.FileWriter(
        params["summaries_dir"] + '/validation')

    tf.compat.v1.global_variables_initializer().run()

    start_step = 1

    if params["start_checkpoint"]:
        models.load_variables_from_checkpoint(sess, params["start_checkpoint"])
        start_step = global_step.eval(session=sess)

    tf.compat.v1.logging.info('Training from step: %d ', start_step)

    # Save graph.pbtxt.
    tf.io.write_graph(sess.graph_def, params["train_dir"],
                      params["model_architecture"] + '.pbtxt')

    # Save list of words.
    with gfile.GFile(
            os.path.join(params["train_dir"],
                         params["model_architecture"] + '_labels.txt'),
            'w') as f:
        f.write('\n'.join(audio_processor.words_list))

    # Training loop.
    training_steps_max = np.sum(training_steps_list)
    for training_step in xrange(start_step, training_steps_max + 1):
        # Figure out what the current learning rate is.
        training_steps_sum = 0
        for i in range(len(training_steps_list)):
            training_steps_sum += training_steps_list[i]
            if training_step <= training_steps_sum:
                learning_rate_value = learning_rates_list[i]
                break
        # Pull the audio samples we'll use for training.
        train_fingerprints, train_ground_truth = audio_processor.get_data(
            params["batch_size"], 0, model_settings, params["background_frequency"],
            params["background_volume"], time_shift_samples, 'training', sess)
        # Run the graph with this batch of training data.
        train_summary, train_accuracy, cross_entropy_value, _, _ = sess.run(
            [
                merged_summaries,
                evaluation_step,
                cross_entropy_mean,
                train_step,
                increment_global_step,
            ],
            feed_dict={
                fingerprint_input: train_fingerprints,
                ground_truth_input: train_ground_truth,
                learning_rate_input: learning_rate_value,
                dropout_rate: 0.5
            })
        train_writer.add_summary(train_summary, training_step)
        tf.compat.v1.logging.debug(
            'Step #%d: rate %f, accuracy %.1f%%, cross entropy %f' %
            (training_step, learning_rate_value, train_accuracy * 100,
             cross_entropy_value))
        is_last_step = (training_step == training_steps_max)
        if (training_step % params["eval_step_interval"]) == 0 or is_last_step:
            tf.compat.v1.logging.info(
                'Step #%d: rate %f, accuracy %.1f%%, cross entropy %f' %
                (training_step, learning_rate_value, train_accuracy * 100,
                 cross_entropy_value))
            set_size = audio_processor.set_size('validation')
            total_accuracy = 0
            total_conf_matrix = None
            for i in xrange(0, set_size, params["batch_size"]):
                validation_fingerprints, validation_ground_truth = (
                    audio_processor.get_data(params["batch_size"], i, model_settings, 0.0,
                                             0.0, 0, 'validation', sess))
                # Run a validation step and capture training summaries for TensorBoard
                # with the `merged` op.
                validation_summary, validation_accuracy, conf_matrix = sess.run(
                    [merged_summaries, evaluation_step, confusion_matrix],
                    feed_dict={
                        fingerprint_input: validation_fingerprints,
                        ground_truth_input: validation_ground_truth,
                        dropout_rate: 0.0
                    })
                validation_writer.add_summary(
                    validation_summary, training_step)
                batch_size = min(params["batch_size"], set_size - i)
                total_accuracy += (validation_accuracy * batch_size) / set_size
                if total_conf_matrix is None:
                    total_conf_matrix = conf_matrix
                else:
                    total_conf_matrix += conf_matrix
            tf.compat.v1.logging.info(
                'Confusion Matrix:\n %s' % (total_conf_matrix))
            tf.compat.v1.logging.info('Step %d: Validation accuracy = %.1f%% (N=%d)' %
                                      (training_step, total_accuracy * 100, set_size))

        # Save the model checkpoint periodically.
        if (training_step % params["save_step_interval"] == 0 or
                training_step == training_steps_max):
            checkpoint_path = os.path.join(params["train_dir"],
                                           params["model_architecture"] + '.ckpt')
            tf.compat.v1.logging.info('Saving to "%s-%d"', checkpoint_path,
                                      training_step)
            saver.save(sess, checkpoint_path, global_step=training_step)

    set_size = audio_processor.set_size('testing')
    tf.compat.v1.logging.info('set_size=%d', set_size)
    total_accuracy = 0
    total_conf_matrix = None
    for i in xrange(0, set_size, params["batch_size"]):
        test_fingerprints, test_ground_truth = audio_processor.get_data(
            params["batch_size"], i, model_settings, 0.0, 0.0, 0, 'testing', sess)
        test_accuracy, conf_matrix = sess.run(
            [evaluation_step, confusion_matrix],
            feed_dict={
                fingerprint_input: test_fingerprints,
                ground_truth_input: test_ground_truth,
                dropout_rate: 0.0
            })
        batch_size = min(params["batch_size"], set_size - i)
        total_accuracy += (test_accuracy * batch_size) / set_size
        if total_conf_matrix is None:
            total_conf_matrix = conf_matrix
        else:
            total_conf_matrix += conf_matrix
    tf.compat.v1.logging.warn('Confusion Matrix:\n %s' % (total_conf_matrix))
    tf.compat.v1.logging.warn('Final test accuracy = %.1f%% (N=%d)' %
                              (total_accuracy * 100, set_size))


if __name__ == '__main__':
    WANTED_WORDS = "yles,alla,parem,vasak"
    
    PREPROCESS = 'micro'

    SAMPLE_RATE = 16000
    CLIP_DURATION_MS = 1000
    WINDOW_SIZE_MS = 30.0
    WINDOW_STRIDE = 20
    FEATURE_BIN_COUNT = 40
    BACKGROUND_FREQUENCY = 0.0 # 0.8
    BACKGROUND_VOLUME_RANGE = 0.0 #0.1
    TIME_SHIFT_MS = 100.0

    model_settings = models.prepare_model_settings(
        len(input_data.prepare_words_list(WANTED_WORDS.split(','))),
        SAMPLE_RATE, CLIP_DURATION_MS, WINDOW_SIZE_MS,
        WINDOW_STRIDE, FEATURE_BIN_COUNT, PREPROCESS)

    lower_band_limit = 0.0 # Float, the lowest frequency included in the filterbanks.
    upper_band_limit = 7500.0 # Float, the highest frequency included in the filterbanks.

    model_settings["upper_band_limit"] = upper_band_limit
    model_settings["lower_band_limit"] = lower_band_limit

    #DATA_URL = 'https://storage.googleapis.com/download.tensorflow.org/data/speech_commands_v0.02.tar.gz'

    DATA_URL = 'https://drive.google.com/uc?export=download&id=1oeGZW_WpkNudyPU-yBhyOJ-71SW4H6rS'
    DATASET_DIR =  'recordings_wav/recordings_wav'

    # Calculate the percentage of 'silence' and 'unknown' training samples required
    # to ensure that we have equal number of samples for each label.
    number_of_labels = WANTED_WORDS.count(',') + 1
    number_of_total_labels = number_of_labels + 2 # for 'silence' and 'unknown' label
    equal_percentage_of_training_samples = int(100.0/(number_of_total_labels))

    SILENT_PERCENTAGE = equal_percentage_of_training_samples
    UNKNOWN_PERCENTAGE = equal_percentage_of_training_samples

    VALIDATION_PERCENTAGE = 10
    TESTING_PERCENTAGE = 10
    LOGS_DIR = 'logs/'
    TRAIN_DIR = "train"

    MODEL_ARCHITECTURE = 'tiny_conv'
    #TRAINING_STEPS = "12000,3000"
    TRAINING_STEPS = "1200,300"
    LEARNING_RATE = "0.001,0.0001"

    EVAL_STEP_INTERVAL = 1000
    SAVE_STEP_INTERVAL = 1000

    VERBOSITY = 'WARN'


    audio_processor = input_data.AudioProcessor(DATA_URL, DATASET_DIR, SILENT_PERCENTAGE, UNKNOWN_PERCENTAGE,
                                                WANTED_WORDS.split(','), VALIDATION_PERCENTAGE, TESTING_PERCENTAGE, 
                                                model_settings, LOGS_DIR)

    params = {
          'data_url':DATA_URL,
          'data_dir':DATASET_DIR,
          'background_volume': 0.1, # How loud the background noise should be, between 0 and 1.
          'background_frequency':0.8,
          'silence_percentage':SILENT_PERCENTAGE,
          'unknown_percentage':UNKNOWN_PERCENTAGE,
          'time_shift_ms':TIME_SHIFT_MS,
          'testing_percentage':TESTING_PERCENTAGE,
          'validation_percentage':VALIDATION_PERCENTAGE,
          'sample_rate':SAMPLE_RATE,
          'clip_duration_ms':CLIP_DURATION_MS,
          'window_size_ms': WINDOW_SIZE_MS,
          'window_stride_ms':WINDOW_STRIDE,
          'feature_bin_count':FEATURE_BIN_COUNT,
          'how_many_training_steps':TRAINING_STEPS,
          'eval_step_interval':EVAL_STEP_INTERVAL,
          'learning_rate':LEARNING_RATE,
          'batch_size':16,
          'summaries_dir':LOGS_DIR,
          'wanted_words':WANTED_WORDS,
          'train_dir':TRAIN_DIR,
          'save_step_interval':SAVE_STEP_INTERVAL,
          'start_checkpoint':"",
          'model_architecture':MODEL_ARCHITECTURE,
          'check_nans':False,
          'quantize':True,
          'preprocess':PREPROCESS,
          'verbosity':VERBOSITY,
          'optimizer':'gradient_descent'}