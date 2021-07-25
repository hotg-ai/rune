import unittest
import math
from rune_py import Normalize, Fft
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
