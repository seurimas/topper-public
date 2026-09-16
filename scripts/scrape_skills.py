#!/usr/bin/env python3
"""
Aetolia Skills Scraper
Scrapes skill pages from aetolia.com and saves as JSON.
Works for general, mercantile, and character class pages.

Actual HTML structure:
  section.ablist-skillset
    h3.ablist-skillset-title  -> skill name
    ul.ablist-list
      li.ablist-ab [data-name, data-info]  (attrs may span multiple lines)
        span.ablist-ab-name    -> ability name
        span.ablist-ab-summary -> one-line summary
        div.ablist-ab-info (hidden)
          div.ablist-ab-info-inner
            h4.ablist-ab-info-title
            div.ablist-ab-desc    -> description HTML
            div.ablist-ab-details -> optional details table (key-value pairs)
"""

import json
import os
import time
import urllib.request
import re
import sys

# Categories to scrape
CATEGORIES = {
    "general": "https://www.aetolia.com/skills-and-abilities/general/",
    "mercantile": "https://www.aetolia.com/skills-and-abilities/merchantile/",
}

CATEGORY_DIRS = {
    "general": "General Skills",
    "mercantile": "Mercantile Skills",
}

OUTPUT_DIR = os.path.join(os.path.dirname(os.path.abspath(__file__)), "output")


def sanitize_filename(name):
    """Make a safe filename from a skill/class name."""
    return re.sub(r'[^\w\s-]', '', name).strip().replace(' ', '_')


def render_ability(ab):
    """Render a single ability as markdown."""
    name = ab.get("name", "Unknown")
    summary = ab.get("summary", "")
    description = ab.get("description", "")
    details = ab.get("details", {})

    lines = []
    lines.append(f"### {name}")
    if summary:
        lines.append(f"*{summary}*")
    lines.append("")

    if description:
        lines.append(description)
        lines.append("")

    if details:
        lines.append("**Details:**")
        lines.append("")
        lines.append("| Stat | Value |")
        lines.append("|------|-------|")
        for key, val in details.items():
            # Escape pipes in values
            key = key.replace('|', '\\|')
            val = val.replace('|', '\\|')
            lines.append(f"| {key} | {val} |")
        lines.append("")

    return '\n'.join(lines)


def render_skill_markdown(skill_name, abilities, category_label, page_url=""):
    """Render a complete markdown file for one skill."""
    lines = []

    # Header
    lines.append(f"# {skill_name}")
    lines.append(f"*{category_label} Skill — Aetolia*")
    lines.append("")
    if page_url:
        lines.append(f"Source: {page_url}")
        lines.append("")

    # Summary table
    lines.append("## Abilities Overview")
    lines.append("")
    lines.append("| Ability | Summary |")
    lines.append("|---------|---------|")
    for ab in abilities:
        name = ab.get("name", "")
        summary = ab.get("summary", "").replace('|', '\\|')
        lines.append(f"| {name} | {summary} |")
    lines.append("")

    # Divider
    lines.append("---")
    lines.append("")

    # Full ability entries
    lines.append("## Ability Details")
    lines.append("")
    for ab in abilities:
        lines.append(render_ability(ab))
        lines.append("---")
        lines.append("")

    return '\n'.join(lines)


def write_skill_files(skills, out_dir, category_label, page_url_fn):
    """Render and write one markdown file per skill into out_dir."""
    os.makedirs(out_dir, exist_ok=True)
    written = []
    for skill_name, abilities in skills.items():
        filename = sanitize_filename(skill_name) + ".md"
        filepath = os.path.join(out_dir, filename)
        md = render_skill_markdown(skill_name, abilities, category_label, page_url=page_url_fn(skill_name))
        with open(filepath, "w", encoding="utf-8") as f:
            f.write(md)
        written.append(filepath)
    return written


def generate_markdown_for_categories(all_data):
    """Write General/Mercantile Skills markdown files from scraped category data."""
    for category, skills in all_data.items():
        dir_name = CATEGORY_DIRS.get(category, category.title())
        out_dir = os.path.join(OUTPUT_DIR, dir_name)
        category_label = dir_name.replace(" Skills", "").replace(" Skill", "")
        written = write_skill_files(
            skills, out_dir, category_label,
            lambda _skill, category=category: f"https://www.aetolia.com/skills-and-abilities/{category}/"
        )
        for filepath in written:
            print(f"  Written: {os.path.relpath(filepath, OUTPUT_DIR)}")


def generate_markdown_for_classes(all_classes):
    """Write Character Classes/<Class>/ markdown files from scraped class data."""
    for class_name, skills in all_classes.items():
        out_dir = os.path.join(OUTPUT_DIR, "Character Classes", class_name)
        written = write_skill_files(
            skills, out_dir, f"{class_name} Class",
            lambda _skill, class_name=class_name: f"https://www.aetolia.com/character-classes/{class_name.lower()}/"
        )
        for filepath in written:
            print(f"  Written: {os.path.relpath(filepath, OUTPUT_DIR)}")


def fetch_page(url):
    """Fetch a URL and return the HTML content."""
    headers = {
        "User-Agent": "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36"
    }
    req = urllib.request.Request(url, headers=headers)
    try:
        with urllib.request.urlopen(req, timeout=30) as resp:
            return resp.read().decode("utf-8", errors="replace")
    except Exception as e:
        print(f"ERROR fetching {url}: {e}")
        return None


def strip_html_tags(html_text):
    """Remove HTML tags and return clean plain text."""
    if not html_text:
        return ""
    # Replace block elements with newlines
    text = re.sub(r'<br\s*/?>', '\n', html_text, flags=re.IGNORECASE)
    text = re.sub(r'</p>', '\n', text, flags=re.IGNORECASE)
    text = re.sub(r'</li>', '\n', text, flags=re.IGNORECASE)
    text = re.sub(r'<p[^>]*>', '', text, flags=re.IGNORECASE)
    # Remove all remaining tags
    text = re.sub(r'<[^>]+>', '', text)
    # Decode HTML entities
    text = text.replace('&amp;', '&').replace('&lt;', '<').replace('&gt;', '>')
    text = text.replace('&quot;', '"').replace('&#39;', "'").replace('&nbsp;', ' ')
    text = re.sub(r'&#(\d+);', lambda m: chr(int(m.group(1))), text)
    text = re.sub(r'&#x([0-9a-fA-F]+);', lambda m: chr(int(m.group(1), 16)), text)
    # Clean up whitespace
    lines = [l.strip() for l in text.split('\n')]
    lines = [l for l in lines if l]
    return '\n'.join(lines)


def parse_details_table(html_text):
    """Extract key-value pairs from the details table HTML."""
    rows = re.findall(r'<tr[^>]*>(.*?)</tr>', html_text, re.DOTALL)
    result = {}
    for row in rows:
        cells = re.findall(r'<td[^>]*>(.*?)</td>', row, re.DOTALL)
        if len(cells) >= 2:
            key = strip_html_tags(cells[0]).strip()
            val = strip_html_tags(cells[1]).strip()
            if key:
                result[key] = val
    return result


def extract_between_tags(html, open_pattern, close_tag):
    """
    Find the first match of open_pattern, then extract until the matching close_tag,
    handling nesting. Returns (content, end_pos) or (None, -1).
    """
    m = re.search(open_pattern, html, re.DOTALL | re.IGNORECASE)
    if not m:
        return None, -1

    start = m.end()
    tag_name = close_tag.lstrip('</')
    depth = 1
    pos = start

    # Simple tag extractor
    tag_re = re.compile(r'<(/?)' + re.escape(tag_name) + r'(\s|>|/)', re.IGNORECASE)
    while pos < len(html) and depth > 0:
        tm = tag_re.search(html, pos)
        if not tm:
            break
        if tm.group(1) == '/':
            depth -= 1
        else:
            depth += 1
        pos = tm.end()

    return html[start:pos - len(close_tag) - 2], pos


def parse_skills_page(html):
    """
    Parse an Aetolia skill page and return a dict of:
      { skill_name: [ { name, summary, description, details }, ... ] }
    """
    skills = {}

    # Split on section.ablist-skillset (the container for each skill group)
    chunks = re.split(r'<section\s+class="ablist-skillset"[^>]*>', html)

    for chunk in chunks[1:]:  # skip content before first skillset
        # Get skill name
        title_m = re.search(r'<h3[^>]*class="ablist-skillset-title"[^>]*>(.*?)</h3>', chunk, re.DOTALL)
        if not title_m:
            continue
        skill_name = strip_html_tags(title_m.group(1)).strip()
        if not skill_name:
            continue

        # Find all li.ablist-ab entries
        # Note: data-name and data-info attributes may span multiple lines
        # The li opening tag looks like:
        #   <li class="ablist-ab" data-name="Foo"\n    data-info="Bar.">
        li_pattern = re.compile(
            r'<li\s+class="ablist-ab"\s+data-name="([^"]*)"[^>]*?data-info="([^"]*)"[^>]*?>',
            re.DOTALL | re.IGNORECASE
        )

        abilities = []
        for li_m in li_pattern.finditer(chunk):
            data_name = li_m.group(1).strip()
            data_info = li_m.group(2).strip()
            li_start = li_m.end()

            # Find the end of this li - look for </li>
            li_end = chunk.find('</li>', li_start)
            if li_end < 0:
                li_content = chunk[li_start:]
            else:
                li_content = chunk[li_start:li_end]

            # Get ability name
            name_m = re.search(r'<span[^>]*class="ablist-ab-name"[^>]*>(.*?)</span>', li_content, re.DOTALL)
            ability_name = strip_html_tags(name_m.group(1)).strip() if name_m else data_name

            # Get summary
            summary_m = re.search(r'<span[^>]*class="ablist-ab-summary"[^>]*>(.*?)</span>', li_content, re.DOTALL)
            summary = strip_html_tags(summary_m.group(1)).strip() if summary_m else data_info

            # Get description from div.ablist-ab-desc
            desc_m = re.search(
                r'<div[^>]*class="ablist-ab-desc"[^>]*>(.*?)</div>\s*</div>',
                li_content, re.DOTALL
            )
            description = ""
            if desc_m:
                desc_html = desc_m.group(1)
                desc_html = re.sub(r'<strong>Description</strong>', '', desc_html)
                description = strip_html_tags(desc_html).strip()

            # Get details table
            details_m = re.search(
                r'<div[^>]*class="ablist-ab-details"[^>]*>(.*?)</tbody>\s*</table>\s*</div>',
                li_content, re.DOTALL
            )
            details = {}
            if details_m:
                details = parse_details_table(details_m.group(1))

            abilities.append({
                "name": ability_name,
                "summary": summary,
                "description": description,
                "details": details
            })

        if abilities:
            skills[skill_name] = abilities
        else:
            print(f"  WARNING: No abilities found for skill '{skill_name}'")

    return skills


def scrape_url(label, url):
    """Scrape any Aetolia skill page (works for general, mercantile, class pages)."""
    print(f"\n{'='*60}")
    print(f"Scraping: {label} -> {url}")
    html = fetch_page(url)
    if not html:
        print(f"  FAILED to fetch {url}")
        return {}

    print(f"  Fetched {len(html):,} bytes")
    skills = parse_skills_page(html)
    print(f"  Found {len(skills)} skills: {list(skills.keys())}")
    for skill_name, abilities in skills.items():
        print(f"    {skill_name}: {len(abilities)} abilities")
    return skills


def scrape_class(class_name):
    """Scrape a character class page. Returns { skill_name: [abilities] }."""
    url = f"https://www.aetolia.com/character-classes/{class_name.lower()}/"
    return scrape_url(class_name, url)


def scrape_all_classes(class_names):
    """Scrape all given character classes. Returns { class_name: { skill_name: [abilities] } }."""
    all_classes = {}
    for name in class_names:
        skills = scrape_class(name)
        all_classes[name] = skills
        time.sleep(1)
    return all_classes


ALL_CHARACTER_CLASSES = [
    "Akkari", "Alchemist", "Archivist", "Ascendril", "Bard", "Bloodborn",
    "Carnifex", "Earthcaller", "Executor", "Indorani", "Infiltrator",
    "Luminary", "Monk", "Oneiromancer", "Praenomen", "Predator", "Ravager",
    "Revenant", "Runecarver", "Sciomancer", "Sentinel", "Shaman",
    "Shapeshifter", "Siderealist", "Sylvan", "Templar", "Teradrim",
    "Tidesage", "Voidseer", "Warden", "Wayfarer", "Zealot"
]


def main():
    all_data = {}

    for category, url in CATEGORIES.items():
        skills = scrape_url(category, url)
        all_data[category] = skills
        time.sleep(1)

    # Save combined JSON
    os.makedirs(OUTPUT_DIR, exist_ok=True)
    output_path = os.path.join(OUTPUT_DIR, "aetolia_skills.json")
    with open(output_path, "w", encoding="utf-8") as f:
        json.dump(all_data, f, indent=2, ensure_ascii=False)
    print(f"\nSaved combined JSON to: {output_path}")

    # Save individual category JSONs
    for category, skills in all_data.items():
        cat_path = os.path.join(OUTPUT_DIR, f"aetolia_{category}.json")
        with open(cat_path, "w", encoding="utf-8") as f:
            json.dump(skills, f, indent=2, ensure_ascii=False)
        print(f"Saved {category} JSON to: {cat_path}")

    # Render per-skill markdown files
    print("\nGenerating General & Mercantile skill markdown files...")
    generate_markdown_for_categories(all_data)

    # Summary
    print("\n" + "="*60)
    print("EXTRACTION SUMMARY")
    print("="*60)
    total_abilities = 0
    for category, skills in all_data.items():
        print(f"\n{category.upper()}:")
        for skill_name, abilities in skills.items():
            print(f"  {skill_name}: {len(abilities)} abilities")
            total_abilities += len(abilities)
    print(f"\nTotal abilities extracted: {total_abilities}")

    return all_data


def main_classes(class_names=None):
    """Scrape character class pages and save JSON + print summary."""
    if class_names is None:
        class_names = ALL_CHARACTER_CLASSES

    all_classes = scrape_all_classes(class_names)

    os.makedirs(OUTPUT_DIR, exist_ok=True)
    output_path = os.path.join(OUTPUT_DIR, "aetolia_classes.json")
    with open(output_path, "w", encoding="utf-8") as f:
        json.dump(all_classes, f, indent=2, ensure_ascii=False)
    print(f"\nSaved classes JSON to: {output_path}")

    # Render per-skill markdown files
    print("\nGenerating Character Classes skill markdown files...")
    generate_markdown_for_classes(all_classes)

    print("\n" + "="*60)
    print("CLASS EXTRACTION SUMMARY")
    print("="*60)
    total = 0
    for class_name, skills in all_classes.items():
        ability_count = sum(len(abs) for abs in skills.values())
        skill_names = list(skills.keys())
        print(f"  {class_name}: {len(skills)} skills ({skill_names}) = {ability_count} abilities")
        total += ability_count
    print(f"\nTotal class abilities: {total}")

    return all_classes


if __name__ == "__main__":
    import sys
    if len(sys.argv) > 1 and sys.argv[1] == "classes":
        # Scrape specific classes or all
        classes = sys.argv[2:] if len(sys.argv) > 2 else None
        main_classes(classes)
    else:
        main()
