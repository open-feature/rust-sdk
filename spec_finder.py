#!/usr/bin/env python
import logging
from typing import Any, Dict, Optional, Set
import urllib.request
import json
import re
import difflib
import os
import sys

# Configure logging
logging.basicConfig(level=logging.INFO, format='%(levelname)s: %(message)s')

SPEC_URL = 'https://raw.githubusercontent.com/open-feature/spec/main/specification.json'
SPEC_PATH = './specification.json'

def _demarkdown(t):
    """Remove markdown-like formatting from text."""
    return t.replace('**', '').replace('`', '').replace('"', '')

def get_spec(force_refresh=False):
    """Fetch the specification, either from a local file or by downloading it."""
    if os.path.exists(SPEC_PATH) and not force_refresh:
        with open(SPEC_PATH, encoding='utf-8') as f:
            data = f.read()
    else:
        try:
            with urllib.request.urlopen(SPEC_URL) as response:
                data = response.read().decode('utf-8')
            with open(SPEC_PATH, 'w', encoding='utf-8') as f:
                f.write(data)
        except Exception as e:
            logging.error("Failed to fetch specification: %s", e)
            sys.exit(1)

    return json.loads(data)

def extract_spec_map(actual_spec: Dict[str, Any]) -> Dict[str, str]:
    """Extract the specification map from the JSON data."""
    spec_map = {}
    for entry in actual_spec['rules']:
        number = re.search(r'[\d.]+', entry['id'])
        if number:
            number = number.group()
            if 'requirement' in entry['machine_id']:
                spec_map[number] = _demarkdown(entry['content'])
        for ch in entry.get('children', []):
            number = re.search(r'[\d.]+', ch['id'])
            if number:
                number = number.group()
                if 'requirement' in ch['machine_id']:
                    spec_map[number] = _demarkdown(ch['content'])
    return spec_map

def parse_rust_files() -> Dict[str, Dict[str, str]]:
    """Parse Rust files and extract specification numbers and corresponding text."""
    repo_specs = {}
    for root, _, files in os.walk(".", topdown=False):
        for name in files:
            if not name.endswith('.rs'):
                continue
            with open(os.path.join(root, name), encoding='utf-8') as f:
                data = f.read()
            for match in re.findall(r'#\[spec\((?P<innards>.*?)\)\]', data.replace('\n', ''), re.MULTILINE | re.DOTALL):
                number_match = re.findall(r'number\s*=\s*"(.*?)"', match)
                if not number_match:
                    continue
                number = number_match[0]
                text_with_concat_chars = re.findall(r'text\s*=\s*(.*)', match)
                try:
                    text = _demarkdown(''.join(text_with_concat_chars) + '"')
                    repo_specs[number] = {'number': number, 'text': text}
                except Exception as e:
                    logging.warning("Skipping %s due to parsing error: %s", match, e)
    return repo_specs

def main(refresh_spec: bool = False, diff_output: bool = False, limit_numbers: Optional[Set[str]] = None) -> None:
    """Main function to compare specifications with Rust tests."""
    actual_spec = get_spec(refresh_spec)
    spec_map = extract_spec_map(actual_spec)
    repo_specs = parse_rust_files()

    missing = set(spec_map.keys())
    bad_num = len(missing)

    for number, entry in sorted(repo_specs.items(), key=lambda x: x[0]):
        if limit_numbers is not None and number not in limit_numbers:
            continue
        if number in spec_map:
            txt = entry['text']
            if txt != spec_map[number]:
                logging.info("%s is bad", number)
                bad_num += 1
                if diff_output:
                    diff = difflib.ndiff([txt], [spec_map[number]])
                    logging.info('\n'.join([li for li in diff if not li.startswith(' ')]))
        else:
            logging.info("%s is defined in our tests, but couldn't find it in the spec", number)

    if missing:
        logging.info('In the spec, but not in our tests:')
        for m in sorted(missing):
            logging.info("%s: %s", m, spec_map[m])

    sys.exit(bad_num)


if __name__ == '__main__':
    import argparse

    parser = argparse.ArgumentParser(description='Parse the spec to make sure our tests cover it')
    parser.add_argument('--refresh-spec', action='store_true', help='Re-download the spec')
    parser.add_argument('--diff-output', action='store_true', help='Print the text differences')
    parser.add_argument('specific_numbers', metavar='num', type=str, nargs='*',
                        help='Limit this to specific numbers')

    args = parser.parse_args()
    main(refresh_spec=args.refresh_spec, diff_output=args.diff_output, limit_numbers=args.specific_numbers)
