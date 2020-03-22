import { Component, OnInit } from '@angular/core';
import {MusicService} from "../music.service";
import {Alarm} from "../alarm";
import {Router} from "@angular/router";

@Component({
  selector: 'app-alarm',
  templateUrl: './alarm.component.html',
  styleUrls: ['./alarm.component.css']
})
export class AlarmComponent implements OnInit {

  schedule: Array<Alarm> = [];
  displayedColumns: string[] = ['sunday', 'monday', 'tuesday', 'wednesday', 'thursday', 'friday', 'saturday', 'start', 'end', 'delete'];

  constructor(private musicService: MusicService, public router: Router) {

  }

  ngOnInit() {
    this.schedule = [];
    this.musicService.getSchedule().subscribe(
      (data) => {
        this.schedule = data
      },
      error => {
        alert(error.toString())
      }
    )
  }

  remove(id: String) {
    this.musicService.deleteSchedule(id).subscribe(
      (data) => this.ngOnInit(),
      error => {
        alert(error.toString())
      }
    )
  }

}
