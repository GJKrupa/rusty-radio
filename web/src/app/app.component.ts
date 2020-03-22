import { Component } from '@angular/core';
import {Observable} from "rxjs";
import {BreakpointObserver, Breakpoints} from "@angular/cdk/layout";
import {map, shareReplay} from "rxjs/operators";
import {MusicService} from "./music.service";
import {Router} from "@angular/router";

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css']
})
export class AppComponent {
  idleTime: number = 60;
  interval;
  time = Date();

  isHandset$: Observable<boolean> = this.breakpointObserver.observe(Breakpoints.Handset)
    .pipe(
      map(result => result.matches),
      shareReplay()
    );

  constructor(private breakpointObserver: BreakpointObserver, private musicService: MusicService, public router: Router) {
  }

  stop() {
    this.musicService.stop()
      .subscribe((it) => { alert("OK") })
  }

  hideCover() {
    console.log("Clicky");
    clearInterval(this.interval);
    document.getElementById("saver1").style.display = "none";
    this.idleTime = 60;
    this.interval = setInterval(() => {
      console.log("ticky " + this.idleTime);
      this.time = Date();
      if(this.idleTime > 0) {
        this.idleTime--;
      }

      if (this.idleTime == 0) {
        document.getElementById("saver1").style.display = "block";
        document.getElementById("saver1").style.background = "#000000";
        clearInterval(this.interval);
      } else if (this.idleTime == 45) {
        document.getElementById("saver1").style.display = "block";
        document.getElementById("saver1").style.background = "rgba(0,0,0,0.8)";
      }
    },1000)
  }
}
