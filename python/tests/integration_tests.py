import unittest
import math
from rune_py import Normalize, Fft, ImageNormalization, Distribution
import numpy as np


class NormalizeTests(unittest.TestCase):
    def test_constructor(self):
        _ = Normalize()

    def test_normalize_empty_list(self):
        norm = Normalize()

        norm([])

    def test_works_with_integers(self):
        norm = Normalize()

        normalized = norm([0, 1, 2, 3, 4, 5])

        self.assertEqual(normalized, [0.0, 0.2, 0.4, 0.6, 0.8, 1.0])

    def test_already_normalized(self):
        norm = Normalize()
        src = [0.0, 0.5, 1.0]

        normalized = norm(src)

        self.assertEqual(normalized, src)

    def test_cant_normalize_strings(self):
        norm = Normalize()
        src = [1.0, "oops"]

        with self.assertRaises(TypeError):
            norm(src)


class FftTest(unittest.TestCase):
    def test_constructor(self):
        _ = Fft(360)

    def test_calculate_spectrum(self):
        fft = Fft(360)
        samples = [round(math.sin(i * 2) * 100) for i in range(0, 1960)]

        spectrum = fft(samples)

        self.assertEqual(1960, len(spectrum))


class ImageNormalizationTest(unittest.TestCase):
    def test_constructor_and_setters(self):
        norm = ImageNormalization(red=(5.0, 1.5), blue=Distribution(10.0, 2.5))

        self.assertEqual(norm.red, Distribution(5.0, 1.5))
        self.assertEqual(norm.blue, Distribution(10.0, 2.5))
        self.assertEqual(norm.green, Distribution(0.0, 1.0))

    def test_normalizing(self):
        image = np.array(
            [
                [[1], [4], [7], [10]],
                [[2], [5], [8], [11]],
                [[3], [6], [9], [12]],
            ],
            # [
            #     [[1, 2, 3], [4, 5, 6], [7, 8, 9], [10, 11, 12]],
            # ],
            dtype="float32",
        )
        mean = image.mean(axis=(1, 2))
        std = image.std(axis=(1, 2))
        print("shape", image.shape)
        print("mean", mean)
        print("std", std)
        norm = ImageNormalization(
            red=(mean[0], std[0]), green=(mean[1], std[1]), blue=(mean[2], std[2])
        )
        should_be = normalize_with_numpy(image, mean, std)

        got = norm(image)

        print(Distribution(10, 0.75))

        self.assertEqual(got, should_be)


def normalize_with_numpy(image: np.ndarray, mean: np.ndarray, std: np.ndarray):
    image = np.copy(image)

    image[0, ...] -= mean[0]
    image[1, ...] -= mean[1]
    image[2, ...] -= mean[2]
    image[0, ...] /= std[0]
    image[1, ...] /= std[1]
    image[2, ...] /= std[2]

    return image
