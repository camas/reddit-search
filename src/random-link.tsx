import * as React from 'react';

/** Renders a random link from a list. Very cool and funny */
export class RandomLink extends React.Component {
  static links = [
    ["Zulu Landing - Ichinchilla", "https://open.spotify.com/album/0unOh093kByDqDN22lfCaG"],
    ["https://i.imgur.com/PRcD6vi.jpg"],
    ["PRESS RELEASE: 37SIGNALS VALUATION TOPS $100 BILLION AFTER BOLD VC INVESTMENT", "https://signalvnoise.com/posts/1941-press-release-37signals-valuation-tops-100-billion-after-bold-vc-investment"],
    ["Pork Scratching Reviews", "https://www.porkscratchingworld.com/Bags1.html"],
    ["AmIUnique", "https://amiunique.org/"],
    ["\"I read all the change logs I am a God\"", "https://twitter.com/SwiftOnSecurity/status/1169757846298230784"],
    ["Gears", "https://ciechanow.ski/gears/"],
    ["The Great Trolley Problem Dump", "https://imgur.com/gallery/QXF8B"],
    ["Cities of the Future", "https://www.nationalgeographic.com/magazine/2019/04/see-sustainable-future-city-designed-for-people-and-nature/"],
    ["Buy a stock, wait until it goes up, and then sell it. If it doesn't go up, then don't have bought it.", "text"],
    ["https://pbs.twimg.com/media/EQvo8N-UcAEyZcx?format=png"],
    ["https://www.bouncingdvdlogo.com/"],
    ["Lemmings in javascript", "https://www.elizium.nu/scripts/lemmings/"],
    ["https://jspaint.app/"],
    ["Map of UK greenbelts", "https://alasdair.carto.com/viz/c1925a82-9670-11e4-ab1a-0e853d047bba/embed_map"],
  ];

  static entry = RandomLink.links[Math.floor(Math.random() * RandomLink.links.length)];

  render() {
    let entry = RandomLink.entry;
    let name = entry[0];
    let url;
    if (entry.length == 1) {
      url = entry[0];
    } else {
      url = entry[1];
    }

    if (url == "text") {
      return <>"{name}"</>
    }
    return <a className="text-blue-400 hover:text-blue-600" href={url}>{name}</a>
  }
}
