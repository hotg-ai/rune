import unittest
import math
import rune_py
import numpy as np


class SmokeTest(unittest.TestCase):
    def test_it_works(self):
        assert rune_py.__AUTHORS__ == "The Rune Developers <developers@hotg.ai>"
