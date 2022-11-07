import { Component, OnInit } from '@angular/core';
import { ActivatedRoute } from '@angular/router';

@Component({
  selector: 'app-client-toolbar',
  templateUrl: './client-toolbar.component.html',
  styleUrls: ['./client-toolbar.component.css']
})
export class ClientToolbarComponent implements OnInit {

  constructor(
    private route: ActivatedRoute
  ) { }

  ngOnInit(): void {
    console.debug("ClientToolbarComponent::ngOnIni()", this.route.snapshot.url);
  }

}
