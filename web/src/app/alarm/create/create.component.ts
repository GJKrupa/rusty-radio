import { Component, OnInit } from '@angular/core';
import {Alarm} from "../../alarm";
import {MusicService} from "../../music.service";
import {Router} from "@angular/router";

@Component({
  selector: 'app-alarm-create',
  templateUrl: './create.component.html',
  styleUrls: ['./create.component.css']
})
export class CreateComponent implements OnInit {

  alarm: Alarm;

  stations = [
    {name: "BBC Radio 1", url: "http://bbcmedia.ic.llnwd.net/stream/bbcmedia_radio1_mf_p"},
    {name: "BBC Radio 1xtra", url: "http://bbcmedia.ic.llnwd.net/stream/bbcmedia_radio1xtra_mf_p"},
    {name: "BBC Radio 2", url: "http://bbcmedia.ic.llnwd.net/stream/bbcmedia_radio2_mf_p"},
    {name: "BBC Radio 3", url: "http://bbcmedia.ic.llnwd.net/stream/bbcmedia_radio3_mf_p"},
    {name: "BBC Radio 5 Live", url: "http://bbcmedia.ic.llnwd.net/stream/bbcmedia_radio5live_mf_p"},
    {name: "BBC Radio 6 Music", url: "http://bbcmedia.ic.llnwd.net/stream/bbcmedia_6music_mf_p"},
    {name: "BBC Asian Network", url: "http://bbcmedia.ic.llnwd.net/stream/bbcmedia_asianet_mf_p"},
    {name: "BBC Radio 4FM", url: "http://bbcmedia.ic.llnwd.net/stream/bbcmedia_radio4fm_mf_p"},
    {name: "BBC Radio 4LW", url: "http://bbcmedia.ic.llnwd.net/stream/bbcmedia_radio4lw_mf_p"},
    {name: "BBC Radio 4 Extra", url: "http://bbcmedia.ic.llnwd.net/stream/bbcmedia_radio4extra_mf_p"},
    {
      name: "BBC Radio 5 Live Sportsball Extra",
      url: "http://a.files.bbci.co.uk/media/live/manifesto/audio/simulcast/hls/uk/sbr_high/ak/bbc_radio_five_live_sports_extra.m3u8"
    },
    {name: "BBC World Service UK stream", url: "http://bbcwssc.ic.llnwd.net/stream/bbcwssc_mp1_ws-eieuk"},
    {name: "BBC World Service News stream", url: "http://bbcwssc.ic.llnwd.net/stream/bbcwssc_mp1_ws-einws"},
  ];

  constructor(private musicService: MusicService, public router: Router) {

  }

  ngOnInit() {
    this.alarm = new Alarm();
  }

  ok() {
    this.musicService.addSchedule(this.alarm).subscribe(
      (data) => this.router.navigateByUrl("/alarm"),
      (err) => alert(err.toString())
    )
  }

}
