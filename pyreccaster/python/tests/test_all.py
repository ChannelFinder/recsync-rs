import pyreccaster


def test_sum_as_string() -> None:
    assert pyreccaster.sum_as_string(1, 1) == "2"  # type: ignore[attr-defined]
