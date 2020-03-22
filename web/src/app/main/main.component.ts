import {Component, Injectable} from '@angular/core';
import {MusicService} from "../music.service";
import {Time} from "@angular/common";

@Component({
  selector: 'app-main',
  templateUrl: './main.component.html',
  styleUrls: ['./main.component.css']
})
export class MainComponent {
  time = Date();
  interval;

  constructor() {
    this.interval = setInterval(() => {
      this.time = Date();
    }, 1000)
  }
}
