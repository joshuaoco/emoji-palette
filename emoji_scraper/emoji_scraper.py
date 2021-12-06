# This script scrapes image files representing emojis for various fonts from the Unicode website.
# These images are not owned by the Unicode Consortium, but they helpfully have them there.

# This could be replaced by actually just getting the real fonts and extracting them
# But I decided writing the web scraper would be easier than getting the fonts from Apple!

import requests
import base64
from bs4 import BeautifulSoup
import os

UNICODE_CHART_URL = "https://www.unicode.org/emoji/charts/full-emoji-list.html"


def get_image_data(images, index):
    """Get the binary image data from INDEX of IMAGES"""
    try:
        raw_image_string = images[index].img.get("src")
    except (IndexError, AttributeError):
        return None

    # This string contains all the data for the image, as well
    # as metadata about the image itself
    # Here we remove the metadata  
    base64_image_data = raw_image_string.split(",")[1]

    # What's left is image data, which is encoded in a format, base64
    # To get this back into the true "raw" format, we decode it using
    # a built in standard library function
    image_data = base64.b64decode(base64_image_data)

    return image_data

def save_image_data(codepoint, version, data):
    """Saves DATA as a .png in version/codepoint.png"""
    filename = f"{version}/{codepoint}.png"
    with open(filename, 'wb') as f:
        f.write(data)


# Create folders to which images will be saved
try:
    os.mkdir("apple")
except FileExistsError:
    pass

try:
    os.mkdir("apple")
except FileExistsError:
    pass
try:
    os.mkdir("google")
except FileExistsError:
    pass
try:
    os.mkdir("facebook")
except FileExistsError:
    pass
try:
    os.mkdir("windows")
except FileExistsError:
    pass
try:
    os.mkdir("twitter")
except FileExistsError:
    pass

try:
    os.mkdir("samsung")
except FileExistsError:
    pass

# Download the HTML of the webpage
page = requests.get(UNICODE_CHART_URL).content

# Parse it with BeautifulSoup into a format that it can search easily
soup = BeautifulSoup(page, "html.parser")

# Get a set of the rows on which we want to operate
rows = soup.find_all("tr")[3:] # Skip the headers

for row in rows:
    try:
        index = row.find("td", class_="rchars").text
    except AttributeError:
        continue

    print(f"Processing row {index}")

    code = row.find("td", class_="code").a.text

    images = row.find_all("td", class_="andr")

    if apple := get_image_data(images, 0):
        save_image_data(code, "apple", apple)

    if google := get_image_data(images, 1):
        save_image_data(code, "google", google)

    if facebook := get_image_data(images, 2):
        save_image_data(code, "facebook", facebook)

    if windows := get_image_data(images, 3):
        save_image_data(code, "windows", windows)

    if twitter := get_image_data(images, 4):
        save_image_data(code, "twitter", twitter)

    if samsung := get_image_data(images, 6):
        save_image_data(code, "samsung", samsung)
