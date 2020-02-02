import { Injectable } from '@angular/core';
import {HttpClient} from "@angular/common/http";

@Injectable({
  providedIn: 'root'
})
export class MusicService {

  constructor(private http: HttpClient) { }

  play(url) {
    return this.http.post("http://pi-radio.localdomain:8080/play", {
      url: url
    })
  }

  stop() {
    return this.http.post("http://pi-radio.localdomain:8080/stop", "")
  }
}
