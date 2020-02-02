import {Component, Injectable} from '@angular/core';
import {MusicService} from "../music.service";

@Component({
  selector: 'app-main',
  templateUrl: './main.component.html',
  styleUrls: ['./main.component.css']
})
export class MainComponent {
  /** Based on the screen size, switch from standard to one column per row */
  stations = [
    {name: "BBC Radio 1", url: "http://bbcmedia.ic.llnwd.net/stream/bbcmedia_radio1_mf_p"},
    {name: "BBC Radio 1xtra", url: "http://bbcmedia.ic.llnwd.net/stream/bbcmedia_radio1xtra_mf_p"},
    {name: "BBC Radio 2", url: "http://bbcmedia.ic.llnwd.net/stream/bbcmedia_radio2_mf_p"},
    {name: "BBC Radio 3", url: "http://bbcmedia.ic.llnwd.net/stream/bbcmedia_radio3_mf_p"},
    {name: "BBC Radio 4FM", url: "http://bbcmedia.ic.llnwd.net/stream/bbcmedia_radio4fm_mf_p"},
    {name: "BBC Radio 4LW", url: "http://bbcmedia.ic.llnwd.net/stream/bbcmedia_radio4lw_mf_p"},
    {name: "BBC Radio 4 Extra", url: "http://bbcmedia.ic.llnwd.net/stream/bbcmedia_radio4extra_mf_p"},
    {name: "BBC Radio 5 Live", url: "http://bbcmedia.ic.llnwd.net/stream/bbcmedia_radio5live_mf_p"},
    {
      name: "BBC Radio 5 Live Sportsball Extra",
      url: "http://a.files.bbci.co.uk/media/live/manifesto/audio/simulcast/hls/uk/sbr_high/ak/bbc_radio_five_live_sports_extra.m3u8"
    },
    {name: "BBC Radio 6 Music", url: "http://bbcmedia.ic.llnwd.net/stream/bbcmedia_6music_mf_p"},
    {name: "BBC Asian Network", url: "http://bbcmedia.ic.llnwd.net/stream/bbcmedia_asianet_mf_p"},
    {name: "BBC World Service UK stream", url: "http://bbcwssc.ic.llnwd.net/stream/bbcwssc_mp1_ws-eieuk"},
    {name: "BBC World Service News stream", url: "http://bbcwssc.ic.llnwd.net/stream/bbcwssc_mp1_ws-einws"},
  ];

  constructor(private musicService: MusicService) {}

  play(url) {
    this.musicService.play(url)
      .subscribe((it) => { alert("OK") })
  }

  stop() {
    this.musicService.stop()
      .subscribe((it) => { alert("OK") })
  }
}
