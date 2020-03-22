import {Injectable} from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {Alarm} from "./alarm";

@Injectable({
  providedIn: 'root'
})
export class MusicService {

  constructor(private http: HttpClient) { }

  play(url) {
    return this.http.post("http://localhost:8080/play", {
      url: url
    })
  }

  stop() {
    return this.http.post("http://localhost:8080/stop", "")
  }

  getSchedule() {
    return this.http.get<Array<Alarm>>("http://localhost:8080/schedule")
  }

  deleteSchedule(id: String) {
    return this.http.delete<string>(`http://localhost:8080/schedule/${id}`)
  }

  addSchedule(alarm: Alarm) {
    return this.http.post<string>(`http://localhost:8080/schedule`, alarm)
  }
}
