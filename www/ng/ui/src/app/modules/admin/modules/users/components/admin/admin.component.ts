import { Component, OnInit } from '@angular/core';
import { TitleService } from 'src/app/services/title.service';

@Component({
  selector: 'app-admin',
  templateUrl: './admin.component.html',
  styleUrls: ['./admin.component.css']
})
export class AdminComponent implements OnInit {

  constructor(
    private title: TitleService
  ) {
    this.title.set_title("Administer Users");
  }

  ngOnInit(): void {
    console.log("AdminComponent::ngOnInit()");
  }

}
